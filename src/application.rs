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

use super::*;
use super::window::*;

use crate::input_handling::*;

use std::collections::HashMap;

use winit::event::*;
use winit::event_loop::*;

/* Application */

pub enum RunMode {
    Wait,
    WaitTimeout(std::time::Duration),
    WaitUntill(std::time::Instant),
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
    windows: HashMap<winit::window::WindowId, Window>,
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

fn line_to_pixels(lines: Delta) -> Delta {
    Delta::new(lines.x * 120.0, lines.y * 120.0)
}

impl<'a> Application<'a> {
    fn new(app_data: &'a mut ApplicationData, event_loop: &'a EventLoopWindowTarget<()>) -> Self {
        Self {
            app_data,
            event_loop,
        }
    }

    pub fn push_window(&mut self, window: Window) {
        self.app_data.windows.insert(window.id(), window);
    }

    pub fn run<C>()
    where
        C: ApplicationController,
    {
        let mut event_loop = EventLoop::new();
        let mut controller = C::new();
        let mut app = ApplicationData::new();

        event_loop
            .unwrap()
            .run(move |event, event_loop| {
                match event {
                    Event::NewEvents(cause) => match cause {
                        StartCause::Init => {
                            controller.on_start(&mut Application::new(&mut app, &event_loop));
                        }
                        StartCause::Poll => {
                            controller.on_poll();

                            for window in app.windows.values_mut() {
                                window.poll();
                            }
                        }
                        StartCause::ResumeTimeReached { .. } => {
                            controller.on_poll();

                            for window in app.windows.values_mut() {
                                window.poll();
                            }
                        }
                        StartCause::WaitCancelled { .. } => {
                            controller.on_poll();

                            for window in app.windows.values_mut() {
                                window.poll();
                            }
                        }
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
                                // WindowEvent::ReceivedCharacter(char) => {}
                                WindowEvent::Focused(_) => {}
                                WindowEvent::KeyboardInput { event, .. } => {
                                    let event = input_handling::KeyEvent::new(
                                        event.physical_key,
                                        event.state,
                                    );
                                    window.on_key(&event);
                                }
                                WindowEvent::ModifiersChanged(_) => {}
                                WindowEvent::Ime(_) => {}
                                WindowEvent::CursorMoved { position, .. } => {
                                    let event = CursorMovedEvent::new(position.into());
                                    window.on_cursor_moved(&event);
                                }
                                WindowEvent::CursorEntered { .. } => {}
                                WindowEvent::CursorLeft { .. } => {}
                                WindowEvent::MouseWheel { delta, .. } => {
                                    let delta = match delta {
                                        MouseScrollDelta::LineDelta(x, y) => Delta::new(x, y),
                                        MouseScrollDelta::PixelDelta(pos) => {
                                            Delta::new(pos.x as f32, pos.y as f32)
                                        }
                                    };
                                    let event = MouseWheelEvent::new(delta);
                                    window.on_mouse_wheel(&event);
                                }
                                WindowEvent::MouseInput { button, state, .. } => {
                                    let event = MouseButtonEvent::new(button, state);
                                    window.on_mouse_button(&event);
                                }
                                WindowEvent::TouchpadMagnify { .. } => {}
                                WindowEvent::SmartMagnify { .. } => {}
                                WindowEvent::TouchpadRotate { .. } => {}
                                WindowEvent::TouchpadPressure { .. } => {}
                                WindowEvent::AxisMotion { .. } => {}
                                WindowEvent::Touch(_) => {}
                                WindowEvent::ScaleFactorChanged { .. } => {}
                                WindowEvent::ThemeChanged(_) => {}
                                WindowEvent::Occluded(_) => {}
                                WindowEvent::ActivationTokenDone { .. } => {}
                                WindowEvent::RedrawRequested => {
                                    window.redraw_requested();
                                }
                            }
                        } else {
                            panic!("Event for unknown window");
                        }
                    }
                    Event::DeviceEvent { .. } => {}
                    Event::UserEvent(_) => {}
                    Event::Suspended => {}
                    Event::Resumed => {}
                    Event::AboutToWait => {}
                    Event::LoopExiting => {}
                    Event::MemoryWarning => {}
                }

                match controller.run_mode() {
                    RunMode::Wait => {
                        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait)
                    }
                    RunMode::WaitTimeout(duration) => event_loop
                        .set_control_flow(winit::event_loop::ControlFlow::wait_duration(duration)),
                    RunMode::WaitUntill(instant) => event_loop
                        .set_control_flow(winit::event_loop::ControlFlow::WaitUntil(instant)),
                    RunMode::Poll => {
                        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll)
                    }
                }

                if match controller.exit_mode() {
                    ExitMode::Explicit => controller.should_exit(),
                    ExitMode::LastWindowClosed => app.windows.is_empty(),
                } {
                    event_loop.exit();
                }
            })
            .unwrap();
    }
}

/* Window */
