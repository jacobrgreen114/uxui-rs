use freetype as ft;
use freetype::ffi::FT_UShort;
use std::cell::{RefCell, UnsafeCell};

use glm::Vec3;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Output;

// #[cfg(target_os = "windows")]
// const SYSTEM_FONT_PATH: &str = "C:/Windows/Fonts/";

#[cfg(target_os = "linux")]
const SYSTEM_FONT_PATH: &str = "/usr/share/fonts/";

#[cfg(target_os = "macos")]
const SYSTEM_FONT_PATH: &str = "/System/Library/Fonts/";

static mut FREETYPE_LIBRARY: Option<ft::Library> = None;

fn create_freetype_library() -> ft::Library {
    ft::Library::init().unwrap()
}

pub(crate) fn get_freetype_library() -> &'static ft::Library {
    unsafe { FREETYPE_LIBRARY.get_or_insert_with(create_freetype_library) }
}

static mut FONT_CACHE: Option<HashMap<String, FontFamily>> = None;

fn cache_fonts<P: AsRef<Path>>(cache: &mut HashMap<String, FontFamily>, path: P) {
    let library = get_freetype_library();

    for font_path in std::fs::read_dir(path).unwrap() {
        let mut face = match font_path {
            Ok(entry) => {
                let file_type = entry.file_type().unwrap();
                let path = entry.path();

                if !file_type.is_file() || !path.extension().unwrap().eq_ignore_ascii_case("ttf") {
                    continue;
                }
                match library.new_face(path, 0) {
                    Ok(face) => face,
                    Err(err) => {
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

        let family = if let Some(family) = cache.get_mut(&key) {
            family
        } else {
            cache.insert(
                key.clone(),
                FontFamily {
                    name: family_name,
                    fonts: Vec::new(),
                },
            );
            cache.get_mut(&key).unwrap()
        };

        family.fonts.push(Font::new(face));
    }
}

#[cfg(target_os = "windows")]
fn create_font_cache() -> HashMap<String, FontFamily> {
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

pub fn get_font_cache() -> &'static mut HashMap<String, FontFamily> {
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
            let family_name = family_name.to_lowercase();
            if let Some(family) = cache.get(&family_name) {
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

#[derive(Debug)]
pub struct Glyph {}

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
    face: ft::Face,
    style: FontStyle,
    glyphs: UnsafeCell<HashMap<char, Box<Glyph>>>,
}

impl Font {
    pub fn new(mut face: ft::Face) -> Self {
        let os2 = ft::tt_os2::TrueTypeOS2Table::from_face(&mut face).unwrap();

        let style = face.style_flags();

        Self {
            face,
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
        }
    }

    pub fn get_glyph(&self, codepoint: char) -> Result<&Glyph, ()> {
        if let Some(glyph) = unsafe { &*self.glyphs.get() }.get(&codepoint) {
            unsafe { Ok(&*(glyph.deref() as *const Glyph)) }
        } else {
            self.face
                .load_glyph(
                    self.face.get_char_index(codepoint as usize),
                    ft::face::LoadFlag::RENDER,
                )
                .unwrap();

            todo!();
            Err(())
        }
    }
}

// pub trait Distance {
//     type Output;
//     fn distance(&self, other: &Self) -> Output;
// }
//
// fn find_closest<It>(iter: It, value: &It::Item) -> Option<It::Item>
// where
//     It: Iterator<Item: impl Distance<Output: impl PartialOrd>>,
// {
//     let mut best_match: Option<It::Item> = None;
//     let mut best_distance: Option<Output> = None;
//
//     for item in iter {
//         let distance = value.distance(&item);
//         if let Some(_) = best_match {
//             unsafe {
//                 if distance < best_distance.unwrap() {
//                     best_match = Some(item);
//                     best_distance = Some(distance);
//                 }
//             }
//         } else {
//             best_match = Some(item);
//             best_distance = Some(distance);
//         }
//     }
//
//     best_match
// }

#[derive(Debug)]
pub struct FontFamily {
    name: String,
    fonts: Vec<Font>,
}

impl FontFamily {
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
