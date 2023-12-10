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

extern crate freetype;
extern crate glm;
extern crate image;
extern crate lazy_static;
extern crate num_traits;
extern crate wgpu;
extern crate winit;

mod application;
mod component;
pub mod controls;
mod drawing;
pub mod font;
mod gfx;
pub mod input_handling;
pub mod layouts;
mod scene;
mod window;

pub use self::application::*;
pub use self::scene::*;
pub use self::window::*;

use std::ops::*;
use std::sync::Arc;

use glm::Vec4;
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
    pub const MIN: Self = Self::new(f32::MIN, f32::MIN);

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

impl Mul<Self> for Size {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.width * rhs.width, self.height * rhs.height)
    }
}

impl Div<Self> for Size {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.width / rhs.width, self.height / rhs.height)
    }
}

impl Neg for Size {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.width, -self.height)
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

    pub fn relative_to(&self, other: Point) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Add<Size> for Point {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        Self::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl Sub<Size> for Point {
    type Output = Self;

    fn sub(self, rhs: Size) -> Self::Output {
        Self::new(self.x - rhs.width, self.y - rhs.height)
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
        let x = self.pos.x + (self.size.width - size.width) / 2.0;
        let y = self.pos.y + (self.size.height - size.height) / 2.0;
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
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
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

#[derive(Debug, Copy, Clone)]
pub struct Dimension {
    pub desired: Length,
    pub min: f32,
    pub max: f32,
}

impl Dimension {
    pub const fn new(desired: Length, min: f32, max: f32) -> Self {
        Self { desired, min, max }
    }

    pub const fn fit() -> Self {
        Self::new(Length::Fit, f32::MIN, f32::MAX)
    }

    pub const fn fill() -> Self {
        Self::new(Length::Fill, f32::MIN, f32::MAX)
    }

    pub const fn fixed(pixels: f32) -> Self {
        Self::new(Length::Fixed(pixels), pixels, pixels)
    }

    pub const fn default() -> Self {
        Self::fill()
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Self::default()
    }
}

// todo : consider implenting aspect ratio locking sizing mode (ex. images)
#[derive(Debug, Default, Copy, Clone)]
pub struct Sizing {
    pub width: Dimension,
    pub height: Dimension,
}

impl Sizing {
    pub const fn new(width: Dimension, height: Dimension) -> Self {
        Self { width, height }
    }

    pub const fn fit() -> Self {
        Self::new(Dimension::fit(), Dimension::fit())
    }

    pub const fn fill() -> Self {
        Self::new(Dimension::fill(), Dimension::fill())
    }

    pub const fn fit_min_max(min: Size, max: Size) -> Self {
        Self {
            width: Dimension::new(Length::Fit, min.width, max.width),
            height: Dimension::new(Length::Fit, min.height, max.height),
        }
    }

    pub const fn fixed(size: Size) -> Self {
        Self::new(Dimension::fixed(size.width), Dimension::fixed(size.height))
    }

    pub const fn default() -> Self {
        Self::fit()
    }
}

#[derive(Debug)]
pub enum BindableString {
    Static(String),
    // Binding(StringPropertyBinding),
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

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment::Left
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Top
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Alignment {
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}

pub trait Builder<T: component::Component>: Sized {
    fn build(self) -> T;
    fn build_boxed(self) -> Box<T> {
        Box::new(self.build())
    }
}

/**
 * Initializes the UXUI library in a multithreaded manner.
 * If this function is not called, the library will be initialized using lazy statics.
 */
pub fn initialize() {
    let t = std::thread::spawn(|| {
        font::initialize();
    });
    gfx::initialize();
    t.join().unwrap();
}
