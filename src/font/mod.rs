/*
  Copyright 2023 Jacob Green

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

use freetype::ffi::{FT_Size_Metrics, FT_UShort};
use {freetype as ft, Size};

use std::cell::*;

use gfx::*;
use wgpu;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::*;
use lazy_static::lazy_static;
use Point;

// #[cfg(target_os = "windows")]
// const SYSTEM_FONT_PATH: &str = "C:/Windows/Fonts/";

#[cfg(target_os = "linux")]
const SYSTEM_FONT_PATH: &str = "/usr/share/fonts/";

#[cfg(target_os = "macos")]
const SYSTEM_FONT_PATH: &str = "/System/Library/Fonts/";

const DPI: u32 = 96;

pub(crate) const PT_PER_EM: f32 = 12.0;

const RENDER_SIZE_EM: f32 = 8.0;
const RENDER_SIZE_PT: f32 = RENDER_SIZE_EM * PT_PER_EM;

pub(crate) fn calculate_font_scale(size: FontSize) -> f32 {
    size.as_em() / RENDER_SIZE_EM
}

static mut FREETYPE_LIBRARY: Option<ft::Library> = None;

fn create_freetype_library() -> ft::Library {
    ft::Library::init().unwrap()
}

pub(crate) fn get_freetype_library() -> &'static ft::Library {
    unsafe { FREETYPE_LIBRARY.get_or_insert_with(create_freetype_library) }
}

#[derive(Debug, Copy, Clone)]
pub struct FontSize {
    size_em: f32,
}

impl FontSize {
    pub fn pt(pt: f32) -> Self {
        Self {
            size_em: pt / PT_PER_EM,
        }
    }

    pub fn px(px: f32) -> Self {
        todo!()
    }

    pub fn em(em: f32) -> Self {
        Self { size_em: em }
    }

    pub fn as_pt(&self) -> f32 {
        self.size_em * PT_PER_EM
    }

    pub fn as_px(&self) -> f32 {
        todo!()
    }

    pub fn as_em(&self) -> f32 {
        self.size_em
    }
}

static mut FONT_CACHE: Option<HashMap<Box<str>, FontFamily>> = None;

trait FontFileExt {
    fn is_font_file(&self) -> bool;
}

impl<T> FontFileExt for T
where
    T: AsRef<Path>,
{
    fn is_font_file(&self) -> bool {
        let p = self.as_ref();
        p.is_file()
            && (p.extension().unwrap().eq_ignore_ascii_case("ttf")
                || p.extension().unwrap().eq_ignore_ascii_case("otf"))
    }
}

fn cache_fonts<P: AsRef<Path>>(cache: &mut HashMap<Box<str>, FontFamily>, path: P) {
    let library = get_freetype_library();

    for font_path in std::fs::read_dir(path).unwrap() {
        let face = match &font_path {
            Ok(entry) => {
                let path = entry.path();

                if !path.is_font_file() {
                    continue;
                }
                match library.new_face(path, 0) {
                    Ok(face) => face,
                    Err(err) => {
                        #[cfg(debug_assertions)]
                        eprintln!("Error loading font {}: {}", entry.path().display(), err);
                        continue;
                    }
                }
            }
            Err(err) => {
                panic!("Error reading font directory: {}", err)
            }
        };

        let family_name = face.family_name().unwrap();
        let key = family_name.to_lowercase();

        let family = if let Some(family) = cache.get_mut(key.as_str()) {
            family
        } else {
            cache.insert(key.clone().into(), FontFamily::new(family_name.as_str()));
            cache.get_mut(key.as_str()).unwrap()
        };

        family
            .fonts
            .push(Font::new(font_path.unwrap().path().as_path(), face));
    }
}

#[cfg(target_os = "windows")]
fn create_font_cache() -> HashMap<Box<str>, FontFamily> {
    let mut cache = HashMap::new();

    let system_font_path = PathBuf::from(std::env::var("WINDIR").unwrap()).join("Fonts");
    let user_font_path =
        PathBuf::from(std::env::var("LOCALAPPDATA").unwrap()).join("Microsoft\\Windows\\Fonts");
    if system_font_path.exists() {
        cache_fonts(&mut cache, system_font_path);
    }
    if user_font_path.exists() {
        cache_fonts(&mut cache, user_font_path);
    }

    cache
}

pub fn get_font_cache() -> &'static mut HashMap<Box<str>, FontFamily> {
    unsafe { FONT_CACHE.get_or_insert_with(create_font_cache) }
}

#[derive(Debug, Clone)]
pub enum FontQuery<'a> {
    FamilyName(&'a str),
    FontType(FontCatagory),
    FamilyWithFallback(&'a str, FontCatagory),
}

#[derive(Debug, Clone)]
pub struct BestFontQuery<'a> {
    pub query: FontQuery<'a>,
    pub style: FontStyle,
}

pub fn find_best_font(query: &BestFontQuery) -> Result<&'static Font, ()> {
    let cache = get_font_cache();

    let mut best_font: Option<&Font> = None;

    match query.query {
        FontQuery::FamilyName(family_name) => {
            let key = family_name.to_lowercase();
            if let Some(family) = cache.get(key.as_str()) {
                let font = match family.find_best_match(query.style) {
                    Ok(font) => font,
                    Err(error) => return Err(error),
                };
                best_font = Some(font);
            }
        }
        FontQuery::FontType(font_type) => {
            todo!();
        }
        FontQuery::FamilyWithFallback(family_name, font_type) => {
            todo!()
        }
    }

    match best_font {
        Some(font) => Ok(font),
        None => Err(()),
    }
}

fn align_size(size: usize, alignment: usize) -> usize {
    if size % alignment == 0 {
        size
    } else {
        size / alignment * alignment + alignment
    }
}

#[derive(Debug)]
pub struct Glyph {
    texture: Option<wgpu::Texture>,
    texture_view: Option<wgpu::TextureView>,
    bitmap_size: Size,
    bitmap_offset: Point,
    metrics: ft::GlyphMetrics,
}

impl Glyph {
    fn new(glyph: &ft::GlyphSlot) -> Self {
        let device = get_device();

        let bitmap = glyph.bitmap();

        let texture = if bitmap.buffer().len() == 0 {
            None
        } else {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Glyph Texture"),
                size: wgpu::Extent3d {
                    width: bitmap.width() as u32,
                    height: bitmap.rows() as u32,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[wgpu::TextureFormat::R8Unorm],
            });

            get_queue().write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: Default::default(),
                    aspect: Default::default(),
                },
                bitmap.buffer(),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bitmap.pitch() as u32),
                    rows_per_image: Some(bitmap.rows() as u32),
                },
                texture.size(),
            );

            Some(texture)
        };

        let texture_view = texture
            .as_ref()
            .map(|texture| texture.create_view(&Default::default()));

        Self {
            texture,
            texture_view,
            bitmap_size: Size::new(bitmap.width() as f32, bitmap.rows() as f32),
            bitmap_offset: Point::new(glyph.bitmap_left() as f32, glyph.bitmap_top() as f32),
            metrics: glyph.metrics(),
        }
    }

    pub fn advance(&self) -> f32 {
        self.metrics.horiAdvance as f32 / 64.0
    }

    pub fn size(&self) -> Size {
        Size::new(
            self.metrics.width as f32 / 64.0,
            self.metrics.height as f32 / 64.0,
        )
    }

    pub fn bearing(&self) -> Point {
        Point::new(
            self.metrics.horiBearingX as f32 / 64.0,
            self.metrics.horiBearingY as f32 / 64.0,
        )
    }

    pub fn texture_size(&self) -> Size {
        self.bitmap_size
    }

    pub(crate) fn texture(&self) -> Option<&wgpu::Texture> {
        self.texture.as_ref()
    }
    pub(crate) fn texture_view(&self) -> Option<&wgpu::TextureView> {
        self.texture_view.as_ref()
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct FontStyle {
    pub weight: FontWeight,
    pub width: FontWidth,
    pub type_: FontType,
}

impl FontStyle {
    fn vectorize(&self) -> glm::Vec3 {
        glm::vec3(
            (self.width as i32 - FontWidth::Normal as i32) as f32 * 11.0,
            self.type_ as u8 as f32 * 7.0,
            (self.weight as i32 - FontWeight::Normal as i32) as f32 / 100.0 * 5.0,
        )
    }
}

#[derive(Debug)]
pub struct Font {
    path: Box<Path>,
    face: RwLock<Option<ft::Face>>,
    style: FontStyle,
    glyphs: UnsafeCell<HashMap<char, Pin<Box<Glyph>>>>,
    metrics: FT_Size_Metrics,
}

impl Font {
    pub fn new(path: &Path, mut face: ft::Face) -> Self {
        let os2 = ft::tt_os2::TrueTypeOS2Table::from_face(&mut face).unwrap();

        let style = face.style_flags();

        face.set_char_size(
            (RENDER_SIZE_PT * 64.0) as isize,
            (RENDER_SIZE_PT * 64.0) as isize,
            DPI,
            DPI,
        )
        .unwrap();

        Self {
            path: path.into(),
            face: RwLock::new(None),
            style: FontStyle {
                weight: FontWeight::from(os2.us_weight_class()),
                width: FontWidth::from(os2.us_width_class()),
                type_: if style.contains(ft::face::StyleFlag::ITALIC) {
                    FontType::Italic
                } else {
                    FontType::Normal
                },
            },
            glyphs: UnsafeCell::new(HashMap::new()),
            metrics: face.size_metrics().unwrap(),
        }
    }

    pub fn ascent(&self) -> f32 {
        self.metrics.ascender as f32 / 64.0
    }

    pub fn descent(&self) -> f32 {
        self.metrics.descender as f32 / 64.0
    }

    pub fn line_height(&self) -> f32 {
        self.metrics.height as f32 / 64.0
    }

    pub fn get_glyph(&self, codepoint: char) -> Result<&Glyph, ()> {
        let glyph = if let Some(glyph) = unsafe { &mut *self.glyphs.get() }.get(&codepoint) {
            glyph
        } else {
            self.load();

            let lock = self.face.write().unwrap();
            let face = lock.as_ref().unwrap();

            face.set_char_size(
                (RENDER_SIZE_PT * 64.0) as isize,
                (RENDER_SIZE_PT * 64.0) as isize,
                DPI,
                DPI,
            )
            .unwrap();

            face.load_char(codepoint as usize, ft::face::LoadFlag::RENDER)
                .unwrap();

            let glyph_slot = face.glyph();

            // note: this is a hack to get the glyph to render as an sdf
            // todo: add 'SDF_RENDER_MODE_SDF' to freetype-rs and use that instead
            if !codepoint.is_whitespace() {
                glyph_slot
                    .render_glyph(unsafe { std::mem::transmute(5) })
                    .unwrap();
            }

            let glyph = Box::pin(Glyph::new(glyph_slot));
            let cache = unsafe { &mut *self.glyphs.get() };
            cache.insert(codepoint, glyph);
            cache.get(&codepoint).unwrap()
        };

        Ok(glyph.deref())
    }

    fn load(&self) {
        let mut lock = self.face.write().unwrap();
        match lock.deref_mut() {
            Some(_) => return,
            None => {
                let mut face = get_freetype_library()
                    .new_face(self.path.as_ref(), 0)
                    .unwrap();
                // face.set_char_size(0, 16 * 64, DPI, DPI).unwrap();
                *lock.deref_mut() = Some(face);
            }
        }
    }

    fn unload(&self) {
        let mut lock = self.face.write().unwrap();
        match lock.deref_mut() {
            Some(_) => {
                *lock.deref_mut() = None;
            }
            None => return,
        }
    }
}

#[derive(Debug)]
pub struct FontFamily {
    name: Box<str>,
    fonts: Vec<Font>,
}

impl FontFamily {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            fonts: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fonts(&self) -> &[Font] {
        &self.fonts
    }

    fn find_best_match(&self, query: FontStyle) -> Result<&Font, ()> {
        let query_vector = query.vectorize();

        let mut best_font: Option<&Font> = None;
        let mut best_distance = 0.0f32;

        for font in &self.fonts {
            let delta = glm::builtin::distance(query_vector, font.style.vectorize());
            if let Some(_) = best_font {
                if delta < best_distance {
                    best_font = Some(font);
                    best_distance = delta;
                }
            } else {
                best_font = Some(font);
                best_distance = delta;
            }
        }

        match best_font {
            Some(font) => Ok(font),
            None => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum FontCatagory {
    Serif,
    SansSerif,
    Monospace,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum FontType {
    Normal,
    Italic,
    Oblique,
}

impl Default for FontType {
    fn default() -> Self {
        FontType::Normal
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
    ExtraBlack = 1000,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

impl From<FT_UShort> for FontWeight {
    fn from(value: FT_UShort) -> Self {
        match value {
            ..=149 => FontWeight::Thin,
            150..=249 => FontWeight::ExtraLight,
            250..=349 => FontWeight::Light,
            350..=449 => FontWeight::Normal,
            450..=549 => FontWeight::Medium,
            550..=649 => FontWeight::SemiBold,
            650..=749 => FontWeight::Bold,
            750..=849 => FontWeight::ExtraBold,
            850.. => FontWeight::Black,
            // 950.. => FontWeight::ExtraBlack,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum FontWidth {
    UltraCondensed = 1,
    ExtraCondensed = 2,
    Condensed = 3,
    SemiCondensed = 4,
    Normal = 5,
    SemiExpanded = 6,
    Expanded = 7,
    ExtraExpanded = 8,
    UltraExpanded = 9,
}

impl Default for FontWidth {
    fn default() -> Self {
        FontWidth::Normal
    }
}

impl From<FT_UShort> for FontWidth {
    fn from(value: FT_UShort) -> Self {
        match value {
            ..=1 => FontWidth::UltraCondensed,
            2 => FontWidth::ExtraCondensed,
            3 => FontWidth::Condensed,
            4 => FontWidth::SemiCondensed,
            5 => FontWidth::Normal,
            6 => FontWidth::SemiExpanded,
            7 => FontWidth::Expanded,
            8 => FontWidth::ExtraExpanded,
            9.. => FontWidth::UltraExpanded,
        }
    }
}

pub(crate) fn initialize() {
    get_font_cache();
}
