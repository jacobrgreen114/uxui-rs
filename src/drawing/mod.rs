mod rect;
mod text;

pub use self::rect::*;
pub use self::text::*;

use crate::gfx::*;

use std::mem::size_of;
use std::ops::{Deref, DerefMut};

use glm::ext::*;
use glm::*;
use num_traits::identities::One;

use wgpu;
use Rect;

pub trait Drawable {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub struct DrawingContext<'a> {
    render_pass: wgpu::RenderPass<'a>,
}

impl<'a> DrawingContext<'a> {
    pub(crate) fn new(render_pass: wgpu::RenderPass<'a>) -> Self {
        Self { render_pass }
    }

    #[inline]
    pub fn draw<D>(&mut self, drawable: &'a D)
    where
        D: Drawable,
    {
        drawable.draw(&mut self.render_pass)
    }
}

struct StagingBuffer<T>
where
    T: Sized,
{
    buffer: wgpu::Buffer,
    phantom: std::marker::PhantomData<T>,
}

impl<T> StagingBuffer<T>
where
    T: Sized,
{
    pub fn new_initialized(data: T) -> Self {
        let buffer = get_device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uxui Staging Buffer"),
            size: size_of::<T>() as u64,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
            mapped_at_creation: true,
        });

        let buffer_slice = buffer.slice(..);
        let mapped_data = buffer_slice.get_mapped_range_mut().as_ptr() as *mut T;
        unsafe {
            *mapped_data = data;
        }
        buffer.unmap();

        Self {
            buffer,
            phantom: std::marker::PhantomData,
        }
    }
}

struct Buffer<T> {
    buffer: wgpu::Buffer,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Buffer<T>
where
    T: Sized,
{
    pub fn new(usage: wgpu::BufferUsages) -> Self {
        let buffer = get_device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uxui Buffer"),
            size: size_of::<T>() as u64,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn new_initialized(usage: wgpu::BufferUsages, data: T) -> Self {
        let buffer = get_device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uxui Buffer"),
            size: size_of::<T>() as u64,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        let buffer_slice = buffer.slice(..);
        let mapped_data = buffer_slice.get_mapped_range_mut().as_ptr() as *mut T;
        unsafe {
            *mapped_data = data;
        }
        buffer.unmap();

        Self {
            buffer,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn update(&mut self, data: T) {
        let staging_buffer = StagingBuffer::new_initialized(data);

        let mut encoder = get_device().create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(
            &staging_buffer.buffer,
            0,
            &self.buffer,
            0,
            size_of::<T>() as u64,
        );
        get_queue().submit(Some(encoder.finish()));
    }
}

impl<T> AsRef<wgpu::Buffer> for Buffer<T>
where
    T: Sized,
{
    fn as_ref(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

pub struct UniformBuffer<T>
where
    T: Sized,
{
    buffer: Buffer<T>,
}

impl<T> UniformBuffer<T>
where
    T: Sized,
{
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(wgpu::BufferUsages::UNIFORM),
        }
    }

    pub fn new_initialized(data: T) -> Self {
        Self {
            buffer: Buffer::new_initialized(wgpu::BufferUsages::UNIFORM, data),
        }
    }

    pub fn update(&mut self, data: T) {
        self.buffer.update(data);
    }
}

impl<T> AsRef<wgpu::Buffer> for UniformBuffer<T>
where
    T: Sized,
{
    fn as_ref(&self) -> &wgpu::Buffer {
        self.buffer.as_ref()
    }
}

fn model_projection(rect: Rect, rotation: f32) -> Mat4 {
    let mut mat = Mat4::one();
    mat = translate(&mat, vec3(rect.pos.x as f32, rect.pos.y as f32, 0.0));
    mat = rotate(&mat, rotation, vec3(0.0, 0.0, 1.0));
    mat = scale(
        &mat,
        vec3(rect.size.width as f32, rect.size.height as f32, 0.0),
    );
    mat
}

static UNIFORM_BINDING_LAYOUT_DESCRIPTOR: wgpu::BindGroupLayoutDescriptor =
    wgpu::BindGroupLayoutDescriptor {
        label: Some("Rectangle Binding Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    };

static mut UNIFORM_BINDING_LAYOUT: Option<wgpu::BindGroupLayout> = None;

fn create_uniform_binding_layout() -> wgpu::BindGroupLayout {
    get_device().create_bind_group_layout(&UNIFORM_BINDING_LAYOUT_DESCRIPTOR)
}

pub(crate) fn get_uniform_binding_layout() -> &'static wgpu::BindGroupLayout {
    unsafe { UNIFORM_BINDING_LAYOUT.get_or_insert_with(create_uniform_binding_layout) }
}

static RECT_VERT_SPIRV: &[u8] = include_bytes!("../../shaders/compiled/rect.vert.spv");
static RECT_FRAG_SPIRV: &[u8] = include_bytes!("../../shaders/compiled/rect.frag.spv");

// static RECT_VERT_SPIRV: &[u32] = include_spirv!("shaders/rect.vert", glsl, vert, vulkan1_0);

// static RECT_FRAG_SPIRV: &[u32] = include_spirv!("shaders/rect.frag", glsl, frag, vulkan1_0);

fn create_rectangle_shader() -> (wgpu::ShaderModule, wgpu::ShaderModule) {
    let vert_descriptor = wgpu::ShaderModuleDescriptor {
        label: Some("Rectangle Vertex Shader"),
        //source: ShaderSource::SpirV(Cow::from(RECT_VERT_SPIRV)),
        source: wgpu::util::make_spirv(RECT_VERT_SPIRV),
    };

    let frag_descriptor = wgpu::ShaderModuleDescriptor {
        label: Some("Rectangle Fragment Shader"),
        //source: ShaderSource::SpirV(Cow::from(RECT_FRAG_SPIRV)),
        source: wgpu::util::make_spirv(RECT_FRAG_SPIRV),
    };

    let vert = get_device().create_shader_module(vert_descriptor);
    let frag = get_device().create_shader_module(frag_descriptor);

    (vert, frag)
}

static mut RECTANGLE_SHADER: Option<(wgpu::ShaderModule, wgpu::ShaderModule)> = None;

fn get_rectangle_vert_shader() -> &'static (wgpu::ShaderModule, wgpu::ShaderModule) {
    unsafe { RECTANGLE_SHADER.get_or_insert_with(create_rectangle_shader) }
}

static mut RECTANGLE_LAYOUT: Option<wgpu::PipelineLayout> = None;

fn create_rectangle_layout() -> wgpu::PipelineLayout {
    let descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("Rectangle Pipeline Layout"),
        bind_group_layouts: &[get_uniform_binding_layout(), get_uniform_binding_layout()],
        push_constant_ranges: &[],
    };

    get_device().create_pipeline_layout(&descriptor)
}

fn get_rectangle_layout() -> &'static wgpu::PipelineLayout {
    unsafe { RECTANGLE_LAYOUT.get_or_insert_with(create_rectangle_layout) }
}

fn create_rectangle_pipeline() -> wgpu::RenderPipeline {
    let rect_shader = get_rectangle_vert_shader();
    let descriptor = wgpu::RenderPipelineDescriptor {
        label: Some("Rectangle Pipeline"),
        layout: Some(get_rectangle_layout()),
        vertex: wgpu::VertexState {
            module: &rect_shader.0,
            entry_point: "main",
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: &rect_shader.1,
            entry_point: "main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    };

    get_device().create_render_pipeline(&descriptor)
}

static mut RECTANGLE_PIPELINE: Option<wgpu::RenderPipeline> = None;

fn get_rectangle_pipeline() -> &'static wgpu::RenderPipeline {
    unsafe { RECTANGLE_PIPELINE.get_or_insert_with(create_rectangle_pipeline) }
}
