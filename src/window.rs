use crate::drawing::*;
use crate::gfx::*;
use crate::scene::*;
use crate::*;
use std::cell::{Ref, RefCell};
use std::ffi::c_void;
use std::ops::Deref;

use glm::ext::*;
use glm::*;
use num_traits::identities::One;

use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::*;

use wgpu::*;

pub trait Window {
    fn id(&self) -> WindowId;
    fn resized(&mut self, size: Size);
    fn moved(&mut self, pos: Point);
    fn redraw_requested(&mut self);
    fn close_requested(&mut self) -> bool;
    fn close(&mut self);
    fn closed(&mut self);
    fn poll(&mut self);
}

pub trait WindowBuilder {
    fn build(self, event_loop: &EventLoopWindowTarget<()>) -> Box<dyn Window>;
}

pub trait WindowController: Sized + 'static {
    fn on_create(&mut self, _window: &UiWindow<Self>) {}

    fn on_resize(&mut self, _window: &UiWindow<Self>, _size: Size) {}

    fn on_moved(&mut self, _window: &UiWindow<Self>, _pos: Point) {}

    fn on_close(&mut self, _window: &UiWindow<Self>) -> bool {
        true
    }

    fn on_closed(&mut self, _window: &UiWindow<Self>) {}

    fn on_poll(&mut self, _window: &UiWindow<Self>) {}
}

pub struct WindowConfig<'a> {
    pub title: Option<&'a str>,
    pub size: Option<Size>,
    pub pos: Option<Point>,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
}

impl Default for WindowConfig<'_> {
    fn default() -> Self {
        Self {
            title: None,
            size: None,
            pos: None,
            resizable: true,
            decorations: true,
            transparent: false,
        }
    }
}

impl WindowConfig<'_> {
    fn to_builder(&self) -> winit::window::WindowBuilder {
        let mut builder = winit::window::WindowBuilder::new()
            .with_visible(false)
            .with_transparent(self.transparent)
            .with_resizable(self.resizable)
            .with_decorations(self.decorations);

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

pub struct UiWindowBuilder<C>
where
    C: WindowController,
{
    builder: winit::window::WindowBuilder,
    phantom: std::marker::PhantomData<C>,
}

impl<C> UiWindowBuilder<C>
where
    C: WindowController,
{
    pub fn new() -> Self {
        Self {
            builder: winit::window::WindowBuilder::default(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.builder = self.builder.with_title(title);
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.builder = self
            .builder
            .with_inner_size(LogicalSize::new(size.width, size.height));
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.builder = self
            .builder
            .with_position(LogicalPosition::new(pos.x, pos.y));
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.builder = self.builder.with_resizable(resizable);
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.builder = self.builder.with_decorations(decorations);
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.builder = self.builder.with_transparent(transparent);
        self
    }
}

impl<C> WindowBuilder for UiWindowBuilder<C>
where
    C: WindowController,
{
    fn build(self, event_loop: &EventLoopWindowTarget<()>) -> Box<dyn Window> {
        todo!();
        // let window = self.builder.build(app.event_loop).unwrap();
        // let surface = unsafe { get_instance().create_surface(&window).unwrap() };
        // let mut s = Box::new(UiWindow {
        //     window: Some(window),
        //     surface: Some(surface),
        //     surface_dirty: true,
        //     controller: RefCell::new(()),
        //     scene: RefCell::new(None),
        // });
        // s.redraw_requested();
        // s.controller.borrow_mut().on_create(&s);
        // s
    }
}

pub struct UiWindow<C>
where
    C: WindowController,
{
    window: Option<winit::window::Window>,
    surface: Option<Surface>,
    surface_dirty: bool,
    controller: RefCell<C>,
    scene: RefCell<Option<Box<dyn SceneInterface>>>,
}

impl<C> UiWindow<C>
where
    C: WindowController,
{
    pub fn new(app: &Application, config: &WindowConfig, controller: C) -> Box<Self> {
        let window = config.to_builder().build(app.event_loop).unwrap();
        let surface = unsafe { get_instance().create_surface(&window).unwrap() };

        let mut this = Box::new(Self {
            window: Some(window),
            surface: Some(surface),
            surface_dirty: true,
            controller: RefCell::new(controller),
            scene: RefCell::new(None),
        });
        this.redraw_requested();
        this.controller.borrow_mut().on_create(&this);
        this
    }

    pub fn show(&self) {
        self.window.as_ref().unwrap().set_visible(true);
    }

    pub fn hide(&self) {
        self.window.as_ref().unwrap().set_visible(false);
    }

    pub fn swap_scene(&self, scene: Box<dyn SceneInterface>) -> Option<Box<dyn SceneInterface>> {
        self.scene.borrow_mut().replace(scene)
    }

    fn update_surface(&mut self) {
        assert!(self.surface_dirty);

        let surface = self.surface.as_ref().unwrap();

        // let capabilities = surface.get_capabilities(get_adapter());
        //
        // let format = find_best_format(&capabilities);
        // let present_mode = find_best_present_mode(&capabilities);
        // let alpha_mode = find_best_alpha_mode(&capabilities);
        let size = self.window.as_ref().unwrap().inner_size();

        let config = surface
            .get_default_config(get_adapter(), size.width, size.height)
            .unwrap();
        // let config = SurfaceConfiguration {
        //     usage: TextureUsages::RENDER_ATTACHMENT,
        //     format,
        //     width: size.width,
        //     height: size.height,
        //     present_mode,
        //     alpha_mode,
        //     view_formats: vec![],
        // };

        surface.configure(get_device(), &config);

        self.surface_dirty = false;
    }
}

fn find_best_format(capabilities: &SurfaceCapabilities) -> TextureFormat {
    // todo : implement hdr compatibility
    // if capabilities.formats.contains(&TextureFormat::Rgba16Float) {
    //     return TextureFormat::Rgba16Float;
    // }

    if capabilities
        .formats
        .contains(&TextureFormat::Bgra8UnormSrgb)
    {
        return TextureFormat::Bgra8UnormSrgb;
    }

    *capabilities.formats.first().unwrap()
}

fn find_best_present_mode(capabilities: &SurfaceCapabilities) -> PresentMode {
    if capabilities.present_modes.contains(&PresentMode::Mailbox) {
        return PresentMode::Mailbox;
    }

    *capabilities.present_modes.first().unwrap()
}

fn find_best_alpha_mode(capabilities: &SurfaceCapabilities) -> CompositeAlphaMode {
    *capabilities.alpha_modes.first().unwrap()
}

impl<C> Window for UiWindow<C>
where
    C: WindowController,
{
    fn id(&self) -> WindowId {
        self.window.as_ref().unwrap().id()
    }

    fn resized(&mut self, size: Size) {
        self.controller.borrow_mut().on_resize(self, size);
        self.surface_dirty = true;

        if let Some(scene) = self.scene.borrow_mut().as_mut() {
            scene.on_canvas_size_changed(size);
        }

        #[cfg(target_os = "macos")]
        self.window.as_ref().unwrap().request_redraw();
    }

    fn moved(&mut self, pos: Point) {
        self.controller.borrow_mut().on_moved(self, pos);
    }

    fn redraw_requested(&mut self) {
        if self.surface_dirty {
            self.update_surface();
        }

        let mut scene = self.scene.borrow_mut();
        if let Some(scene) = scene.as_mut() {
            scene.update_layout(self.window.as_ref().unwrap().inner_size().into());
        }

        let surface = self.surface.as_ref().unwrap();

        let texture = surface.get_current_texture().unwrap();
        let view = texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let device = get_device();
        let mut encoder = device.create_command_encoder(&Default::default());

        let size = self.window.as_ref().unwrap().inner_size();

        let buffer = UniformBuffer::new_initialized(UniformRenderInfo::new(size.into()));

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uxui Render Info Bind Group"),
            layout: get_uniform_binding_layout(),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(buffer.as_ref().as_entire_buffer_binding()),
            }],
        });

        {
            let clear_color: wgpu::Color = match scene.as_ref() {
                Some(scene) => scene.get_background_color().into(),
                None => wgpu::Color::BLACK,
            };

            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Uxui Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_bind_group(0, &bind_group, &[]);

            let mut drawing_context = DrawingContext::new(render_pass);

            if let Some(scene) = scene.as_ref() {
                scene.draw(&mut drawing_context);
            }
        }

        let command = encoder.finish();

        get_queue().submit(Some(command));
        texture.present();
    }

    fn close_requested(&mut self) -> bool {
        self.controller.borrow_mut().on_close(self)
    }

    fn close(&mut self) {
        self.surface = None;
        self.window = None;
    }

    fn closed(&mut self) {
        self.controller.borrow_mut().on_closed(self);
    }

    fn poll(&mut self) {
        self.window.as_ref().unwrap().request_redraw();
    }
}

fn orthographic2d(left: f32, right: f32, bottom: f32, top: f32) -> Mat4 {
    orthographic(left, right, bottom, top, -1.0, 1.0)
}

fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let mid_x = (left + right) / 2.0;
    let mid_y = (bottom + top) / 2.0;
    let mid_z = (-near + -far) / 2.0;

    let scale_x = 2.0 / (right - left);
    let scale_y = 2.0 / (top - bottom);
    let scale_z = 2.0 / (far - near);

    let mut mat = Mat4::one();
    mat = scale(&mat, vec3(scale_x, scale_y, scale_z));
    mat = translate(&mat, vec3(-mid_x, -mid_y, -mid_z));
    mat
}

#[repr(packed)]
#[allow(dead_code)]
struct UniformRenderInfo {
    projection: Mat4,
    view: Mat4,
}

impl UniformRenderInfo {
    fn new(render_area: Size) -> Self {
        Self {
            projection: orthographic2d(0.0, render_area.width, render_area.height, 0.0),
            view: look_at(
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 0.0, -1.0),
                vec3(0.0, -1.0, 0.0),
            ),
        }
    }
}
