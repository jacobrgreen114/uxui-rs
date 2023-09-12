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
    fn on_init(&mut self, scene: &Scene<Self>) {
        let model = self.model.clone();

        let root = Column::build()
            .with_width(Length::Fill)
            .with_height(Length::Fill)
            .with_horizontal_alignment(HorizontalAlignment::Center)
            .with_children(vec![
                // Input::builder()
                //     .with_hint("Username")
                //     .with_binding(self.model.username.create_binding())
                //     .build(),
                // Input::builder()
                //     .with_hint("Password")
                //     .with_binding(self.model.password.create_binding())
                //     .build(),
                Button::builder()
                    .with_label("Login")
                    .with_width(Length::Fixed(100.0))
                    .with_height(Length::Fixed(50.0))
                    .with_background(Color::new_rgb(0.0, 1.0, 0.0))
                    .with_action(Box::new(move || {
                        model.on_login();
                    }))
                    .build(),
                Button::builder()
                    .with_label("Login")
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
    fn on_create(&mut self, window: &UiWindow<Self>) {
        let login_scene = Scene::new(LoginSceneController::new());
        window.swap_scene(login_scene);
        window.show();
        println!(
            "Window time to show: {:?}",
            unsafe { TIMER }.unwrap().elapsed()
        );
    }

    fn on_resize(&mut self, window: &UiWindow<Self>, _size: Size) {
        // println!("Window resized to {:?}", size);
    }

    fn on_moved(&mut self, window: &UiWindow<Self>, _pos: Point) {
        // println!("Window moved to {:?}", pos);
    }
}

struct ExampleAppController {}

impl ApplicationController for ExampleAppController {
    fn new() -> Self {
        Self {}
    }

    fn on_start(&mut self, app: &mut Application) {
        app.push_window(UiWindow::new(
            app,
            &WindowConfig {
                title: Some("Example Window"),
                size: Some(Size {
                    width: 640.0,
                    height: 480.0,
                }),
                resizable: false,
                ..Default::default()
            },
            ExampleWindowController::new(),
        ));
    }

    // fn run_mode(&self) -> RunMode {
    //     RunMode::Poll
    // }
}

fn main() {
    unsafe {
        TIMER = Some(Instant::now());
    }

    let best_font = uxui::font::find_best_font(&uxui::font::BestFontQuery {
        query: uxui::font::FontQuery::FamilyName("Segoe UI"),
        style: Default::default(),
    })
    .unwrap();

    println!("Best font: {:#?}", best_font);
    Application::run::<ExampleAppController>()
}
