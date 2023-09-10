extern crate glm;
extern crate inline_spirv;
extern crate num_traits;
extern crate wgpu;
extern crate winit;

mod application;
mod component;
pub mod controls;
mod drawing;
mod gfx;
pub mod layouts;
mod scene;
mod window;

pub use application::*;
pub use component::*;
use glm::Vec4;
pub use scene::*;
use std::sync::Arc;
pub use window::*;

pub struct StringProperty {
    value: Arc<String>,
}

impl StringProperty {
    pub fn new() -> Self {
        Self {
            value: Arc::new(String::new()),
        }
    }

    pub fn create_binding(&self) -> StringPropertyBinding {
        StringPropertyBinding {
            value: self.value.clone(),
        }
    }
}

impl From<&str> for StringProperty {
    fn from(value: &str) -> Self {
        Self {
            value: Arc::new(value.into()),
        }
    }
}

pub struct StringPropertyBinding {
    value: Arc<String>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for Size {
    fn from(size: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            width: size.width as f32,
            height: size.height as f32,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<winit::dpi::PhysicalPosition<i32>> for Point {
    fn from(pos: winit::dpi::PhysicalPosition<i32>) -> Self {
        Self {
            x: pos.x as f32,
            y: pos.y as f32,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

impl Rect {
    pub const fn new(pos: Point, size: Size) -> Self {
        Self { pos, size }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn new_grayscale(gray: f32) -> Self {
        Self {
            r: gray,
            g: gray,
            b: gray,
            a: 1.0,
        }
    }

    pub fn new_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
}

impl Into<Vec4> for Color {
    fn into(self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }
}

pub struct Widget {}

#[derive(Debug, Copy, Clone)]
pub enum Length {
    Fit,
    Fill,
    Fixed(f32),
    // Ratio(f32),
}

impl Default for Length {
    fn default() -> Self {
        Length::Fit
    }
}

#[derive(Debug, Clone)]
pub struct Dimension {
    pub desired: Length,
    pub min: f32,
    pub max: f32,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            desired: Length::default(),
            min: f32::MIN,
            max: f32::MAX,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Sizing {
    pub width: Dimension,
    pub height: Dimension,
}

pub enum BindableString {
    Static(String),
    Binding(StringPropertyBinding),
}

impl Default for BindableString {
    fn default() -> Self {
        Self::Static(String::new())
    }
}
