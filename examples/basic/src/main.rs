#![windows_subsystem = "windows"]

extern crate uxui;

use uxui::*;

struct ExampleWindowController {}

impl WindowController for ExampleWindowController {
    fn new() -> Self {
        Self {}
    }

    fn on_create(&mut self, window: &mut Window) {
        // println!("Window created");
        window.show();
    }

    fn on_resize(&mut self, _size: Size) {
        // println!("Window resized to {:?}", size);
    }

    fn on_moved(&mut self, _pos: Point) {
        // println!("Window moved to {:?}", pos);
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
            size: Some(Size {
                width: 1280,
                height: 720,
            }),
            ..Default::default()
        });
    }
}

fn main() {
    Application::run::<ExampleAppController>()
}
