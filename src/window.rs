use super::*;

use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::*;

pub(crate) trait WindowInterface {
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

pub(crate) struct WindowImpl<C>
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
    pub(crate) fn new(config: &WindowConfig, event_loop: &EventLoopWindowTarget<()>) -> Box<Self> {
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
