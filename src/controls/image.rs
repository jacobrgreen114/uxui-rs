use std::convert::TryFrom;
use std::ops::Deref;
use {image, Size};

use component::*;
use drawing::{DrawingContext, VisualImage};
use input_handling::*;
use {Rect, Sizing};

use gfx::{get_device, get_queue};
use image::EncodableLayout;
use num_traits::Zero;
use std::path::Path;
use wgpu::Extent3d;
use Builder;

/*
   Image Builder
*/

enum ImageSource<'a> {
    File(&'a Path),
    Bytes(&'a [u8]),
}

impl Into<image::DynamicImage> for ImageSource<'_> {
    fn into(self) -> image::DynamicImage {
        match self {
            ImageSource::File(path) => image::open(path).unwrap(),
            ImageSource::Bytes(bytes) => image::load_from_memory(bytes).unwrap(),
        }
    }
}

pub struct ImageBuilder<'a> {
    source: ImageSource<'a>,
    sizing: Sizing,
}

impl ImageBuilder<'_> {
    pub fn with_sizing(mut self, sizing: Sizing) -> Self {
        self.sizing = sizing;
        self
    }
}

impl Builder<Image> for ImageBuilder<'_> {
    fn build(self) -> Image {
        let image = self.source.into();
        let texture = create_texture_from_image(image);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Image {
            sizing: self.sizing,
            texture,
            view,
            visual: None,
        }
    }
}

/*
   Image
*/

pub struct Image {
    sizing: Sizing,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    visual: Option<VisualImage>,
}

impl Image {
    pub fn from_file(path: &Path) -> ImageBuilder {
        ImageBuilder {
            source: ImageSource::File(path.as_ref()),
            sizing: Sizing::default(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> ImageBuilder {
        ImageBuilder {
            source: ImageSource::Bytes(bytes),
            sizing: Sizing::default(),
        }
    }
}

impl Layout for Image {
    fn measure(&mut self, available_size: Size) -> Size {
        let available = self.sizing.calc_available_size(available_size);
        // let required = Size::new(self.texture.width() as f32, self.texture.height() as f32);
        self.sizing.calc_final_size(available, available)
    }

    fn arrange(&mut self, final_rect: Rect) -> Rect {
        self.visual = Some(VisualImage::new(final_rect, &self.view));
        final_rect
    }
}

impl Draw for Image {
    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        context.draw(self.visual.as_ref().unwrap());
    }
}

impl InputHandler for Image {}

impl PreviewInputHandler for Image {}

impl DispatchInput for Image {}

impl Component for Image {}

/*
   Helpers
*/

fn create_texture_from_image(image: image::DynamicImage) -> wgpu::Texture {
    match image {
        image::DynamicImage::ImageLuma8(_) => {
            todo!("Unsupported image format")
        }
        image::DynamicImage::ImageLumaA8(_) => {
            todo!("Unsupported image format")
        }
        image::DynamicImage::ImageRgb8(image) => {
            to_texture_rgba(&image::DynamicImage::ImageRgb8(image).to_rgba8())
        }
        image::DynamicImage::ImageRgba8(image) => to_texture_rgba(&image),
        image::DynamicImage::ImageLuma16(_) => {
            todo!("Unsupported image format")
        }
        image::DynamicImage::ImageLumaA16(_) => {
            todo!("Unsupported image format")
        }
        image::DynamicImage::ImageRgb16(image) => {
            to_texture_rgba16(&image::DynamicImage::ImageRgb16(image).to_rgba16())
        }
        image::DynamicImage::ImageRgba16(image) => to_texture_rgba16(&image),
        image::DynamicImage::ImageRgb32F(_) => {
            todo!("Unsupported image format")
        }

        image::DynamicImage::ImageRgba32F(_) => {
            todo!("Unsupported image format")
        }
        _ => panic!("Unsupported image format"),
    }
}

fn create_texture(size: Extent3d, format: wgpu::TextureFormat) -> wgpu::Texture {
    get_device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Image Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

fn write_texture(texture: &wgpu::Texture, size: Extent3d, data: &[u8], pitch: u32, rows: u32) {
    get_queue().write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: Default::default(),
            aspect: Default::default(),
        },
        data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(pitch),
            rows_per_image: Some(rows),
        },
        size,
    );
}

fn to_texture_rgba(image: &image::RgbaImage) -> wgpu::Texture {
    let (width, height) = image.dimensions();
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let sample_layout = image.sample_layout();

    let texture = create_texture(size, wgpu::TextureFormat::Rgba8Unorm);
    write_texture(
        &texture,
        size,
        image.as_raw().as_bytes(),
        (sample_layout.width as usize * sample_layout.width_stride) as u32,
        sample_layout.height,
    );

    texture
}

fn to_texture_rgba16(image: &image::ImageBuffer<image::Rgba<u16>, Vec<u16>>) -> wgpu::Texture {
    let (width, height) = image.dimensions();
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let sample_layout = image.sample_layout();

    let texture = create_texture(size, wgpu::TextureFormat::Rgba16Float);
    write_texture(
        &texture,
        size,
        image.as_raw().as_bytes(),
        (sample_layout.width as usize * sample_layout.width_stride) as u32,
        sample_layout.height,
    );

    texture
}
