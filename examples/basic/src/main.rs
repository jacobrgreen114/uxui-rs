#![windows_subsystem = "windows"]

extern crate uxui;
extern crate winit;

use std::thread::sleep;
use uxui::*;

struct ExampleWindowController {}

impl WindowController for ExampleWindowController {
    fn new() -> Self {
        Self {}
    }

    fn on_create(&mut self, window: &mut Window) {
        println!("Window created");
        window.show();
    }

    fn on_resize(&mut self, size: Size) {
        println!("Window resized to {:?}", size);
    }

    fn on_moved(&mut self, pos: Point) {
        println!("Window moved to {:?}", pos);
    }
}

struct ExampleAppController {}

impl ApplicationController for ExampleAppController {
    fn new() -> Self {
        Self {}
    }

    fn on_start(&mut self, app: &mut Application) {
        println!("Application started");
        app.create_window::<ExampleWindowController>(&WindowConfig {
            title: Some("Example Window"),
            ..Default::default()
        });
    }
}

fn main() {
    Application::run::<ExampleAppController>()
}
