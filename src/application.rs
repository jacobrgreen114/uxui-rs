use super::window::*;
use super::*;

use std::collections::HashMap;

use winit::event::*;
use winit::event_loop::*;
use winit::platform::run_return::EventLoopExtRunReturn;

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

    fn on_start(&mut self, _app: &mut Application) {}

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
    windows: HashMap<winit::window::WindowId, Box<dyn Window>>,
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
    pub(crate) event_loop: &'a EventLoopWindowTarget<()>,
}

impl<'a> Application<'a> {
    fn new(app_data: &'a mut ApplicationData, event_loop: &'a EventLoopWindowTarget<()>) -> Self {
        Self {
            app_data,
            event_loop,
        }
    }

    pub fn push_window(&mut self, window: Box<dyn Window>) {
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
                    if let Some(window) = app.windows.get_mut(&window_id) {
                        match event {
                            WindowEvent::Resized(size) => {
                                window.resized(size.into());
                            }
                            WindowEvent::Moved(pos) => {
                                window.moved(pos.into());
                            }
                            WindowEvent::CloseRequested => {
                                if window.close_requested() {
                                    window.close()
                                }
                            }
                            WindowEvent::Destroyed => {
                                let mut window = app.windows.remove(&window_id).unwrap();
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
                    if let Some(window) = app.windows.get_mut(&window_id) {
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
