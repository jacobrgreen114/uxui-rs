extern crate freetype;
extern crate glm;
extern crate inline_spirv;
extern crate num_traits;
extern crate wgpu;
extern crate winit;

mod application;
mod binding;
mod component;
pub mod controls;
mod drawing;
pub mod font;
mod gfx;
pub mod input_handling;
pub mod layouts;
mod scene;
mod ui;
mod util;
mod window;

pub use self::application::*;
// pub use self::component::*;
pub use self::binding::*;
pub use self::scene::*;
pub use self::window::*;

use glm::Vec4;

use std::ops::*;
use std::sync::Arc;
use winit::dpi::PhysicalPosition;

pub use num_traits::*;

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
}

impl Add<Self> for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl Sub<Self> for Size {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl Mul<f32> for Size {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.width * rhs, self.height * rhs)
    }
}

impl Div<f32> for Size {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.width / rhs, self.height / rhs)
    }
}

impl num_traits::Zero for Size {
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.width == 0.0 && self.height == 0.0
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

impl From<PhysicalPosition<f64>> for Point {
    fn from(pos: PhysicalPosition<f64>) -> Self {
        Self {
            x: pos.x as f32,
            y: pos.y as f32,
        }
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl num_traits::Zero for Point {
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

pub type Delta = Point;

#[derive(Debug, Default, Copy, Clone)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

impl Rect {
    pub const fn new(pos: Point, size: Size) -> Self {
        Self { pos, size }
    }

    pub fn center(&self) -> Point {
        Point::new(
            self.pos.x + self.size.width / 2.0,
            self.pos.y + self.size.height / 2.0,
        )
    }

    pub fn align_center(&self, size: Size) -> Rect {
        let x = (self.size.width - size.width) / 2.0;
        let y = (self.size.height - size.height) / 2.0;
        Rect::new(Point::new(x, y), size)
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.pos.x
            && point.x <= self.pos.x + self.size.width
            && point.y >= self.pos.y
            && point.y <= self.pos.y + self.size.height
    }

    pub fn intersects(&self, other: Rect) -> bool {
        self.pos.x < other.pos.x + other.size.width
            && self.pos.x + self.size.width > other.pos.x
            && self.pos.y < other.pos.y + other.size.height
            && self.pos.y + self.size.height > other.pos.y
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

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Copy, Clone)]
pub struct Alignment {
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}
