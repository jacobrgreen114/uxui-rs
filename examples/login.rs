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

// Enable this to build a windows desktop app without a console window
#![windows_subsystem = "windows"]

extern crate lazy_static;
extern crate uxui;

use std::rc::Rc;
use std::time::Instant;

use lazy_static::lazy_static;

use uxui::controls::*;
use uxui::layouts::*;
use uxui::*;

const UXUI_LOGO_IMG: &[u8] = include_bytes!("../assets/uxui_logo.png");

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

struct LoginModel {
    username: StringProperty,
    password: StringProperty,
}

impl LoginModel {
    pub fn new() -> Self {
        Self {
            username: StringProperty::new(),
            password: StringProperty::new(),
        }
    }

    pub fn on_login(&self) {
        println!("Login button clicked");
    }
}

struct LoginSceneController {
    model: Rc<LoginModel>,
}

impl LoginSceneController {
    pub fn new() -> Self {
        Self {
            model: Rc::new(LoginModel::new()),
        }
    }
}

impl SceneController for LoginSceneController {
    fn on_init(&mut self, scene: &Scene) {
        let model = self.model.clone();

        let center = Column::builder()
            .with_sizing(Sizing::fill())
            .with_alignment(HorizontalAlignment::Center)
            .with_children(vec![
                Text::builder("Hello, World!\nWelcome to UXUI!").build_boxed(),
                Image::from_bytes(UXUI_LOGO_IMG)
                    .with_sizing(Sizing::fixed(Size::new(64.0, 64.0)))
                    .build_boxed(),
                // Input::builder()
                //     .with_hint("Username")
                //     .with_binding(self.model.username.create_binding())
                //     .build(),
                // Input::builder()
                //     .with_hint("Password")
                //     .with_binding(self.model.password.create_binding())
                //     .build(),
                Button::builder()
                    //.with_content(Text::new("Login"))
                    .with_sizing(Sizing::fixed(Size::new(100.0, 50.0)))
                    .with_background(Color::new_rgb(0.0, 1.0, 0.0))
                    //.with_action(Box::new(move || {
                    //    model.on_login();
                    //}))
                    .build_boxed(),
                // Button::builder()
                //     // .with_label("Login2")
                //     .with_sizing(Sizing::fixed(Size::new(500.0, 50.0)))
                //     .with_background(Color::new_rgb(0.0, 1.0, 1.0))
                //     .build_boxed(),
            ])
            .build_boxed();

        let root = Dock::builder()
            .with_sizing(Sizing::fill())
            .with_center(center)
            .with_top(Text::builder("Top").build_boxed())
            .with_bottom(Text::builder("Bottom").build_boxed())
            .with_left(Text::builder("Left").build_boxed())
            .with_right(Text::builder("Right").build_boxed())
            .build_boxed();

        scene.swap_root(Some(root));
    }
}

struct ExampleWindowController {}

impl ExampleWindowController {
    pub fn new() -> Self {
        Self {}
    }
}

impl WindowController for ExampleWindowController {
    fn on_create(&mut self, window: &Window) {
        let login_scene = Scene::new(LoginSceneController::new());
        window.swap_scene(Some(login_scene));
        window.show();
        println!("Window time to show: {:?}", START_TIME.elapsed());
    }
}

static LOGIN_WINDOW_CONFIG: WindowConfig = WindowConfig {
    title: Some("Login"),
    size: Some(Size {
        width: 800.0,
        height: 600.0,
    }),
    pos: None,
    decorations: true,
    resizable: true,
    transparent: false,
};

struct ExampleAppController {}

impl ApplicationController for ExampleAppController {
    fn new() -> Self {
        Self {}
    }

    fn on_start(&mut self, app: &mut Application) {
        app.push_window(Window::new(
            app,
            &LOGIN_WINDOW_CONFIG,
            ExampleWindowController::new(),
        ));
    }
}

fn main() {
    lazy_static::initialize(&START_TIME);
    // optional
    uxui::initialize();
    Application::run::<ExampleAppController>()
}
