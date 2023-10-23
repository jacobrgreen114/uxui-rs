use std::convert::TryFrom;
use std::ops::Deref;
use {image, Size};

use component::*;
use drawing::{DrawingContext, VisualImage};
use input_handling::{InputHandler, PreviewInputHandler};
use Rect;

use gfx::{get_device, get_queue};
use image::EncodableLayout;
use std::path::Path;
use wgpu::Extent3d;

/*
   Image
*/

pub struct Image {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    visual: Option<VisualImage>,
}

impl Image {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Component {
        Self::from_image(image::open(path).unwrap())
    }

    pub fn from_bytes(bytes: &[u8]) -> Component {
        Self::from_image(image::load_from_memory(bytes).unwrap())
    }

    fn from_image(image: image::DynamicImage) -> Component {
        let texture = create_texture_from_image(image);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        ComponentBuilder::default().build(Image {
            texture,
            view,
            visual: None,
        })
    }
}

impl PreviewInputHandler for Image {}

impl InputHandler for Image {}

impl ComponentController for Image {
    fn measure(&mut self, available_size: Size, children: &[Component]) -> Size {
        available_size
    }

    fn arrange(&mut self, final_rect: Rect, children: &[Component]) -> Rect {
        self.visual = Some(VisualImage::new(final_rect, &self.view));
        final_rect
    }

    fn draw<'a>(&'a self, context: &mut DrawingContext<'a>) {
        context.draw(self.visual.as_ref().unwrap());
    }
}

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
