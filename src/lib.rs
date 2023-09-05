extern crate winit;

use std::collections::HashMap;

use winit::dpi::*;
use winit::event::*;
use winit::event_loop::*;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::*;

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

/* Application */

pub enum RunMode {
    Wait,
    Poll,
}

pub enum ExitMode {
    Explicit,
    LastWindowClosed,
}

pub trait ApplicationController: Sized + 'static {
    fn new() -> Self;

    fn on_start(&mut self, app: &mut Application) {}

    fn on_stop(&mut self) {}

    fn on_poll(&mut self) {}

    fn run_mode(&self) -> RunMode {
        RunMode::Wait
    }

    fn exit_mode(&self) -> ExitMode {
        ExitMode::LastWindowClosed
    }

    fn should_exit(&self) -> bool {
        false
    }
}

struct ApplicationData {
    windows: HashMap<WindowId, Box<dyn WindowInterface>>,
}

impl ApplicationData {
    fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }
}

pub struct Application<'a> {
    app_data: &'a mut ApplicationData,
    event_loop: &'a EventLoopWindowTarget<()>,
}

impl<'a> Application<'a> {
    fn new(app_data: &'a mut ApplicationData, event_loop: &'a EventLoopWindowTarget<()>) -> Self {
        Self {
            app_data,
            event_loop,
        }
    }

    pub fn create_window<WinC>(&mut self, config: &WindowConfig<'a>)
    where
        WinC: WindowController,
    {
        let window = WindowImpl::<WinC>::new(config, self.event_loop);
        self.app_data.windows.insert(window.id(), window);
    }

    pub fn run<C>()
    where
        C: ApplicationController,
    {
        let mut event_loop = EventLoop::new();
        let mut controller = C::new();
        let mut app = ApplicationData::new();

        event_loop.run_return(move |event, event_loop, flow| {
            match event {
                Event::NewEvents(cause) => match cause {
                    StartCause::Init => {
                        controller.on_start(&mut Application::new(&mut app, &event_loop));
                    }
                    StartCause::Poll => {
                        controller.on_poll();
                    }
                    StartCause::ResumeTimeReached { .. } => {}
                    StartCause::WaitCancelled { .. } => {}
                },
                Event::WindowEvent { event, window_id } => {
                    if let Some(mut window) = app.windows.get_mut(&window_id) {
                        match event {
                            WindowEvent::Resized(size) => {
                                window.resized(Size {
                                    width: size.width,
                                    height: size.height,
                                });
                            }
                            WindowEvent::Moved(pos) => {
                                window.moved(Point { x: pos.x, y: pos.y });
                            }
                            WindowEvent::CloseRequested => {
                                if window.close_requested() {
                                    window.close()
                                }
                            }
                            WindowEvent::Destroyed => {
                                let window = app.windows.remove(&window_id).unwrap();
                                window.closed();
                            }
                            WindowEvent::DroppedFile(_) => {}
                            WindowEvent::HoveredFile(_) => {}
                            WindowEvent::HoveredFileCancelled => {}
                            WindowEvent::ReceivedCharacter(_) => {}
                            WindowEvent::Focused(_) => {}
                            WindowEvent::KeyboardInput { .. } => {}
                            WindowEvent::ModifiersChanged(_) => {}
                            WindowEvent::Ime(_) => {}
                            WindowEvent::CursorMoved { .. } => {}
                            WindowEvent::CursorEntered { .. } => {}
                            WindowEvent::CursorLeft { .. } => {}
                            WindowEvent::MouseWheel { .. } => {}
                            WindowEvent::MouseInput { .. } => {}
                            WindowEvent::TouchpadMagnify { .. } => {}
                            WindowEvent::SmartMagnify { .. } => {}
                            WindowEvent::TouchpadRotate { .. } => {}
                            WindowEvent::TouchpadPressure { .. } => {}
                            WindowEvent::AxisMotion { .. } => {}
                            WindowEvent::Touch(_) => {}
                            WindowEvent::ScaleFactorChanged { .. } => {}
                            WindowEvent::ThemeChanged(_) => {}
                            WindowEvent::Occluded(_) => {}
                        }
                    } else {
                        panic!("Event for unknown window");
                    }
                }
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {}
                Event::MainEventsCleared => {}
                Event::RedrawRequested(window_id) => {
                    if let Some(window) = app.windows.get(&window_id) {
                        window.redraw_requested();
                    } else {
                        panic!("Redraw requested for unknown window");
                    }
                }
                Event::RedrawEventsCleared => {}
                Event::LoopDestroyed => {}
            }

            match controller.run_mode() {
                RunMode::Wait => flow.set_wait(),
                RunMode::Poll => flow.set_poll(),
            }

            if match controller.exit_mode() {
                ExitMode::Explicit => controller.should_exit(),
                ExitMode::LastWindowClosed => app.windows.is_empty(),
            } {
                flow.set_exit();
            }
        });
    }
}

/* Window */

trait WindowInterface {
    fn id(&self) -> WindowId;
    fn resized(&mut self, size: Size);
    fn moved(&mut self, pos: Point);
    fn redraw_requested(&self);
    fn close_requested(&self) -> bool;
    fn close(&mut self);
    fn closed(&self);
}

pub trait WindowController: Sized + 'static {
    fn new() -> Self;

    fn title(&self) -> BindableString {
        BindableString::Static("")
    }

    fn on_create(&mut self, _window: &mut Window) {}

    fn on_resize(&mut self, _size: Size) {}

    fn on_moved(&mut self, _pos: Point) {}

    fn on_close(&self) -> bool {
        true
    }

    fn on_closed(&self) {}

    fn on_poll(&mut self) {}
}

pub struct WindowConfig<'a> {
    pub title: Option<&'a str>,
    pub size: Option<Size>,
    pub pos: Option<Point>,
    pub resizable: bool,
    pub visible: bool,
}

impl Default for WindowConfig<'_> {
    fn default() -> Self {
        Self {
            title: None,
            size: None,
            pos: None,
            resizable: true,
            visible: false,
        }
    }
}

impl WindowConfig<'_> {
    fn to_builder(&self) -> WindowBuilder {
        let mut builder = WindowBuilder::new()
            .with_resizable(self.resizable)
            .with_visible(self.visible);

        builder = match self.resizable {
            true => builder.with_enabled_buttons(WindowButtons::all()),
            false => builder.with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE),
        };

        builder = match self.title {
            Some(title) => builder.with_title(title),
            None => builder.with_title(""),
        };

        builder = match self.size {
            Some(size) => builder.with_inner_size(LogicalSize::new(size.width, size.height)),
            None => builder,
        };

        builder = match self.pos {
            Some(pos) => builder.with_position(LogicalPosition::new(pos.x, pos.y)),
            None => builder,
        };

        builder
    }
}

pub struct Window<'a> {
    window: &'a winit::window::Window,
}

impl<'a> Window<'a> {
    fn new(window: &'a winit::window::Window) -> Self {
        Self { window }
    }

    pub fn show(&self) {
        self.window.set_visible(true);
    }
}

struct WindowImpl<C>
where
    C: WindowController,
{
    window: Option<winit::window::Window>,
    controller: C,
}

impl<C> WindowImpl<C>
where
    C: WindowController,
{
    fn new(config: &WindowConfig, event_loop: &EventLoopWindowTarget<()>) -> Box<Self> {
        let window = config.to_builder().build(event_loop).unwrap();
        let mut controller = C::new();
        controller.on_create(&mut Window::new(&window));

        Box::new(Self {
            window: Some(window),
            controller,
        })
    }
}

impl<C> WindowInterface for WindowImpl<C>
where
    C: WindowController,
{
    fn id(&self) -> WindowId {
        self.window.as_ref().unwrap().id()
    }

    fn resized(&mut self, size: Size) {
        self.controller.on_resize(size);
    }

    fn moved(&mut self, pos: Point) {
        self.controller.on_moved(pos);
    }

    fn redraw_requested(&self) {
        // todo : implement redraw logic
    }

    fn close_requested(&self) -> bool {
        self.controller.on_close()
    }

    fn close(&mut self) {
        self.window = None;
    }

    fn closed(&self) {
        self.controller.on_closed();
    }
}
