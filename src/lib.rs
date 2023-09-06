extern crate glm;
extern crate inline_spirv;
extern crate num_traits;
extern crate wgpu;
extern crate winit;

mod application;
pub mod controls;
mod drawing;
mod gfx;
pub mod layouts;
mod window;

pub use application::*;
pub use window::*;

pub enum BindableString<'a> {
    Static(&'a str),
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

pub struct Widget {}
