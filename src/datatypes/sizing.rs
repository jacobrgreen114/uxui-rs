use std::ops::{Add, Div, Mul, Neg, Sub};
use winit::dpi::PhysicalPosition;

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
