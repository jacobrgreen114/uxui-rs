use super::*;
use crate::drawing::*;
use crate::gfx::*;

use glm::ext::*;
use glm::*;
use num_traits::identities::One;

use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::*;

use wgpu::*;

pub(crate) trait WindowInterface {
    fn id(&self) -> WindowId;
    fn resized(&mut self, size: Size);
    fn moved(&mut self, pos: Point);
    fn redraw_requested(&mut self);
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
    pub decorations: bool,
}

impl Default for WindowConfig<'_> {
    fn default() -> Self {
        Self {
            title: None,
            size: None,
            pos: None,
            resizable: true,
            decorations: true,
        }
    }
}

impl WindowConfig<'_> {
    fn to_builder(&self) -> WindowBuilder {
        let mut builder = WindowBuilder::new()
            .with_visible(false)
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
    surface: Option<Surface>,
    surface_dirty: bool,
    controller: C,
}

impl<C> WindowImpl<C>
where
    C: WindowController,
{
    pub(crate) fn new(config: &WindowConfig, event_loop: &EventLoopWindowTarget<()>) -> Box<Self> {
        let mut controller = C::new();

        let window = config.to_builder().build(event_loop).unwrap();
        let surface = unsafe { get_instance().create_surface(&window).unwrap() };

        let mut s = Self {
            window: Some(window),
            surface: Some(surface),
            surface_dirty: true,
            controller,
        };
        s.redraw_requested();
        s.controller
            .on_create(&mut Window::new(&s.window.as_ref().unwrap()));
        Box::new(s)
    }

    fn update_surface(&mut self) {
        assert!(self.surface_dirty);

        let surface = self.surface.as_ref().unwrap();

        let capabilities = surface.get_capabilities(get_adapter());

        let format = find_best_format(&capabilities);
        let present_mode = find_best_present_mode(&capabilities);
        let alpha_mode = find_best_alpha_mode(&capabilities);
        let size = self.window.as_ref().unwrap().inner_size();

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode,
            view_formats: vec![format],
        };

        surface.configure(get_device(), &config);

        self.surface_dirty = false;
    }
}

fn find_best_format(capabilities: &SurfaceCapabilities) -> TextureFormat {
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

impl<C> WindowInterface for WindowImpl<C>
where
    C: WindowController,
{
    fn id(&self) -> WindowId {
        self.window.as_ref().unwrap().id()
    }

    fn resized(&mut self, size: Size) {
        self.controller.on_resize(size);
        self.surface_dirty = true;

        #[cfg(target_os = "macos")]
        self.window.as_ref().unwrap().request_redraw();
    }

    fn moved(&mut self, pos: Point) {
        self.controller.on_moved(pos);
    }

    fn redraw_requested(&mut self) {
        if self.surface_dirty {
            self.update_surface();
        }

        let surface = self.surface.as_ref().unwrap();

        let texture = surface.get_current_texture().unwrap();
        let view = texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let device = get_device();
        let mut encoder = device.create_command_encoder(&Default::default());

        let rect = Rectangle::new(
            Rect {
                pos: Point { x: 0, y: 0 },
                size: Size {
                    width: 1200,
                    height: 700,
                },
            },
            Vec4::new(1.0, 0.0, 1.0, 1.0),
        );

        let size = self.window.as_ref().unwrap().inner_size();

        let buffer = UniformBuffer::new_initialized(UniformRenderInfo::new(Size {
            width: size.width,
            height: size.height,
        }));

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uxui Render Info Bind Group"),
            layout: get_uniform_binding_layout(),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(buffer.as_ref().as_entire_buffer_binding()),
            }],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Uxui Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_bind_group(0, &bind_group, &[]);

            let mut drawing_context = DrawingContext::new(render_pass);
            drawing_context.draw_rectangle(&rect);
        }

        let command = encoder.finish();

        get_queue().submit(Some(command));
        texture.present();
    }

    fn close_requested(&self) -> bool {
        self.controller.on_close()
    }

    fn close(&mut self) {
        self.surface = None;
        self.window = None;
    }

    fn closed(&self) {
        self.controller.on_closed();
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
            projection: orthographic2d(
                0.0,
                render_area.width as f32,
                render_area.height as f32,
                0.0,
            ),
            view: look_at(
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 0.0, -1.0),
                vec3(0.0, -1.0, 0.0),
            ),
        }
    }
}
