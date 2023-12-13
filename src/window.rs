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

use crate::drawing::*;
use crate::gfx::*;
use crate::input_handling::*;
use crate::scene::*;
use crate::*;

use std::cell::{Ref, RefCell};

use glm::ext::*;
use glm::*;
use image::RgbaImage;
use num_traits::identities::One;

use winit::dpi::{LogicalPosition, LogicalSize};
use winit::window::*;

use wgpu::*;

pub trait WindowController {
    fn on_create(&mut self, _window: &Window) {}

    fn on_resize(&mut self, _window: &Window, _size: Size) {}

    fn on_moved(&mut self, _window: &Window, _pos: Point) {}

    fn on_close(&mut self, _window: &Window) -> bool {
        true
    }

    fn on_closed(&mut self, _window: &Window) {}

    fn on_poll(&mut self, _window: &Window) {}
}

pub struct WindowIcon {
    image: RgbaImage,
}

impl WindowIcon {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap().to_rgba8();
        Self { image }
    }
}

pub struct WindowConfig<'a> {
    pub title: Option<&'a str>,
    pub size: Option<Size>,
    pub pos: Option<Point>,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub icon: Option<WindowIcon>,
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
            icon: None,
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

        builder = match &self.icon {
            Some(icon) => builder.with_window_icon(Some(
                Icon::from_rgba(icon.image.to_vec(), icon.image.width(), icon.image.height())
                    .unwrap(),
            )),
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

pub struct Window {
    window: Option<winit::window::Window>,
    surface: Option<Surface>,
    surface_dirty: bool,
    controller: Box<RefCell<dyn WindowController>>,
    scene: RefCell<Option<Scene>>,
    client_area: Size,
}

impl Window {
    pub fn new(
        app: &Application,
        config: &WindowConfig,
        controller: impl WindowController + 'static,
    ) -> Self {
        let window = config.to_builder().build(app.event_loop).unwrap();
        let surface = unsafe { get_instance().create_surface(&window).unwrap() };

        let mut this = Self {
            window: Some(window),
            surface: Some(surface),
            surface_dirty: true,
            controller: Box::new(RefCell::new(controller)),
            scene: RefCell::new(None),
            client_area: Size::default(),
        };
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

    pub fn swap_scene(&self, mut scene: Option<Scene>) -> Option<Scene> {
        match scene.as_mut() {
            Some(scene) => {
                scene.on_active();
            }
            None => {}
        }

        let mut old_scene = self.scene.replace(scene);

        match old_scene.as_mut() {
            Some(old_scene) => {
                old_scene.on_inactive();
            }
            None => {}
        }

        old_scene
    }

    fn update_surface(&mut self) {
        assert!(self.surface_dirty);

        let surface = self.surface.as_ref().unwrap();

        let config = surface
            .get_default_config(
                get_adapter(),
                self.client_area.width as u32,
                self.client_area.height as u32,
            )
            .unwrap();

        surface.configure(get_device(), &config);

        self.surface_dirty = false;
    }

    pub(crate) fn id(&self) -> WindowId {
        self.window.as_ref().unwrap().id()
    }

    pub(crate) fn resized(&mut self, size: Size) {
        self.client_area = size;
        self.controller.borrow_mut().on_resize(self, size);
        self.surface_dirty = true;

        if let Some(scene) = self.scene.borrow_mut().as_mut() {
            scene.on_canvas_size_changed(size);
        }

        #[cfg(target_os = "macos")]
        self.window.as_ref().unwrap().request_redraw();
    }

    pub(crate) fn moved(&mut self, pos: Point) {
        self.controller.borrow_mut().on_moved(self, pos);
    }

    pub(crate) fn redraw_requested(&mut self) {
        if self.client_area.width <= 0.0 || self.client_area.height <= 0.0 {
            return;
        }

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
            layout: &RENDER_INFO_BIND_LAYOUT,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(buffer.as_ref().as_entire_buffer_binding()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&GLOBAL_SAMPLER),
                },
            ],
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
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_bind_group(0, &bind_group, &[]);
            let mut drawing_context =
                DrawingContext::new(render_pass, Rect::new(Point::new(0.0, 0.0), size.into()));

            if let Some(scene) = scene.as_ref() {
                scene.draw(&mut drawing_context);
            }
        }

        let command = encoder.finish();

        self.window.as_ref().unwrap().pre_present_notify();
        get_queue().submit(Some(command));
        texture.present();
    }

    pub(crate) fn close_requested(&mut self) -> bool {
        self.controller.borrow_mut().on_close(self)
    }

    pub(crate) fn close(&mut self) {
        self.surface = None;
        self.window = None;
    }

    pub(crate) fn closed(&mut self) {
        self.controller.borrow_mut().on_closed(self);
    }

    pub(crate) fn poll(&mut self) {
        self.window.as_ref().unwrap().request_redraw();
    }
}

impl InputHandler for Window {
    fn on_key(&mut self, event: &KeyEvent) -> bool {
        match self.scene.borrow_mut().as_mut() {
            Some(scene) => scene.on_key(event),
            None => false,
        }
    }

    fn on_mouse_button(&mut self, event: &MouseButtonEvent) -> bool {
        match self.scene.borrow_mut().as_mut() {
            Some(scene) => scene.on_mouse_button(event),
            None => false,
        }
    }

    fn on_mouse_wheel(&mut self, event: &MouseWheelEvent) -> bool {
        match self.scene.borrow_mut().as_mut() {
            Some(scene) => scene.on_mouse_wheel(event),
            None => false,
        }
    }

    fn on_cursor_moved(&mut self, event: &CursorMovedEvent) -> bool {
        match self.scene.borrow_mut().as_mut() {
            Some(scene) => scene.on_cursor_moved(event),
            None => false,
        }
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

    capabilities.formats.first().unwrap().clone()
}

fn find_best_present_mode(capabilities: &SurfaceCapabilities) -> PresentMode {
    if capabilities.present_modes.contains(&PresentMode::Mailbox) {
        return PresentMode::Mailbox;
    }

    capabilities.present_modes.first().unwrap().clone()
}

fn find_best_alpha_mode(capabilities: &SurfaceCapabilities) -> CompositeAlphaMode {
    capabilities.alpha_modes.first().unwrap().clone()
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
