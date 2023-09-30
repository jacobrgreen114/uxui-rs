#![windows_subsystem = "windows"]

extern crate uxui;

use std::cell::{RefCell, UnsafeCell};
use std::rc::Rc;
use std::sync::RwLock;
use uxui::controls::*;
use uxui::layouts::*;
use uxui::*;

use std::time::Instant;

static mut TIMER: Option<Instant> = None;

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

        let root = Column::build()
            .with_width(Length::Fill)
            .with_height(Length::Fill)
            .with_horizontal_alignment(HorizontalAlignment::Center)
            .with_children(vec![
                Text::new("Hello world!"),
                // Input::builder()
                //     .with_hint("Username")
                //     .with_binding(self.model.username.create_binding())
                //     .build(),
                // Input::builder()
                //     .with_hint("Password")
                //     .with_binding(self.model.password.create_binding())
                //     .build(),
                Button::builder()
                    // .with_label("Login")
                    .with_width(Length::Fixed(100.0))
                    .with_height(Length::Fixed(50.0))
                    .with_background(Color::new_rgb(0.0, 1.0, 0.0))
                    .with_action(Box::new(move || {
                        model.on_login();
                    }))
                    .build(),
                Button::builder()
                    // .with_label("Login2")
                    .with_width(Length::Fixed(500.0))
                    .with_height(Length::Fixed(50.0))
                    .with_background(Color::new_rgb(0.0, 1.0, 1.0))
                    .build(),
            ])
            .build();

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
        println!(
            "Window time to show: {:?}",
            unsafe { TIMER }.unwrap().elapsed()
        );
    }
}

static LOGIN_WINDOW_CONFIG: WindowConfig = WindowConfig {
    title: Some("Login"),
    size: Some(Size {
        width: 640.0,
        height: 480.0,
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
    unsafe {
        TIMER = Some(Instant::now());
    }

    Application::run::<ExampleAppController>()
}
