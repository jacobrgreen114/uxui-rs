mod application;
mod window;

pub use application::*;
pub use window::*;

extern crate winit;

pub enum BindableString<'a> {
    Static(&'a str),
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Size {
    width: u32,
    height: u32,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}
