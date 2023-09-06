use crate::gfx::*;

use std::mem::size_of;
use std::ops::Deref;

use glm::ext::*;
use glm::*;
use num_traits::identities::One;

use wgpu::*;
use Rect;

pub struct DrawingContext<'a> {
    render_pass: RenderPass<'a>,
}

impl<'a> DrawingContext<'a> {
    pub(crate) fn new(render_pass: RenderPass<'a>) -> Self {
        Self { render_pass }
    }

    pub fn draw_rectangle(&mut self, rect: &'a Rectangle) {
        self.render_pass.set_pipeline(get_rectangle_pipeline());
        self.render_pass.set_bind_group(1, &rect.bind_group, &[]);
        self.render_pass.draw(0..6, 0..1)
    }
}

pub struct UniformBuffer<T>
where
    T: Sized,
{
    buffer: Buffer,
    phantom: std::marker::PhantomData<T>,
}

impl<T> UniformBuffer<T>
where
    T: Sized,
{
    pub fn new() -> Self {
        let buffer = get_device().create_buffer(&BufferDescriptor {
            label: Some("Uxui Uniform Buffer"),
            size: size_of::<T>() as u64,
            usage: BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn new_initialized(data: T) -> Self {
        let buffer = get_device().create_buffer(&BufferDescriptor {
            label: Some("Uxui Uniform Buffer"),
            size: size_of::<T>() as u64,
            usage: BufferUsages::UNIFORM,
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

impl<T> AsRef<Buffer> for UniformBuffer<T>
where
    T: Sized,
{
    fn as_ref(&self) -> &Buffer {
        &self.buffer
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

#[repr(packed)]
#[allow(dead_code)]
struct RectangleUniform {
    transform: Mat4,
    color: Vec4,
}

#[allow(dead_code)]
pub struct Rectangle {
    buffer: UniformBuffer<RectangleUniform>,
    bind_group: BindGroup,
}

pub(crate) unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

impl Rectangle {
    pub fn new(rect: Rect, color: Vec4) -> Self {
        let device = get_device();

        let buffer = UniformBuffer::new_initialized(RectangleUniform {
            transform: model_projection(rect, 0.0),
            color,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Rectangle Uniform Bind Group"),
            layout: get_uniform_binding_layout(),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(buffer.as_ref().as_entire_buffer_binding()),
            }],
        });

        Self { buffer, bind_group }
    }
}

static UNIFORM_BINDING_LAYOUT_DESCRIPTOR: BindGroupLayoutDescriptor = BindGroupLayoutDescriptor {
    label: Some("Rectangle Binding Layout"),
    entries: &[BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX_FRAGMENT,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }],
};

static mut UNIFORM_BINDING_LAYOUT: Option<BindGroupLayout> = None;

fn create_uniform_binding_layout() -> BindGroupLayout {
    get_device().create_bind_group_layout(&UNIFORM_BINDING_LAYOUT_DESCRIPTOR)
}

pub(crate) fn get_uniform_binding_layout() -> &'static BindGroupLayout {
    unsafe { UNIFORM_BINDING_LAYOUT.get_or_insert_with(create_uniform_binding_layout) }
}

static RECT_VERT_SPIRV: &[u8] = include_bytes!("../../shaders/compiled/rect.vert.spv");
static RECT_FRAG_SPIRV: &[u8] = include_bytes!("../../shaders/compiled/rect.frag.spv");

// static RECT_VERT_SPIRV: &[u32] = include_spirv!("shaders/rect.vert", glsl, vert, vulkan1_0);

// static RECT_FRAG_SPIRV: &[u32] = include_spirv!("shaders/rect.frag", glsl, frag, vulkan1_0);

fn create_rectangle_shader() -> (ShaderModule, ShaderModule) {
    let vert_descriptor = ShaderModuleDescriptor {
        label: Some("Rectangle Vertex Shader"),
        //source: ShaderSource::SpirV(Cow::from(RECT_VERT_SPIRV)),
        source: util::make_spirv(RECT_VERT_SPIRV),
    };

    let frag_descriptor = ShaderModuleDescriptor {
        label: Some("Rectangle Fragment Shader"),
        //source: ShaderSource::SpirV(Cow::from(RECT_FRAG_SPIRV)),
        source: util::make_spirv(RECT_FRAG_SPIRV),
    };

    let vert = get_device().create_shader_module(vert_descriptor);
    let frag = get_device().create_shader_module(frag_descriptor);

    (vert, frag)
}

static mut RECTANGLE_SHADER: Option<(ShaderModule, ShaderModule)> = None;

fn get_rectangle_vert_shader() -> &'static (ShaderModule, ShaderModule) {
    unsafe { RECTANGLE_SHADER.get_or_insert_with(create_rectangle_shader) }
}

static mut RECTANGLE_LAYOUT: Option<PipelineLayout> = None;

fn create_rectangle_layout() -> PipelineLayout {
    let descriptor = PipelineLayoutDescriptor {
        label: Some("Rectangle Pipeline Layout"),
        bind_group_layouts: &[get_uniform_binding_layout(), get_uniform_binding_layout()],
        push_constant_ranges: &[],
    };

    get_device().create_pipeline_layout(&descriptor)
}

fn get_rectangle_layout() -> &'static PipelineLayout {
    unsafe { RECTANGLE_LAYOUT.get_or_insert_with(create_rectangle_layout) }
}

fn create_rectangle_pipeline() -> RenderPipeline {
    let rect_shader = get_rectangle_vert_shader();
    let descriptor = RenderPipelineDescriptor {
        label: Some("Rectangle Pipeline"),
        layout: Some(get_rectangle_layout()),
        vertex: VertexState {
            module: &rect_shader.0,
            entry_point: "main",
            buffers: &[],
        },
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(FragmentState {
            module: &rect_shader.1,
            entry_point: "main",
            targets: &[Some(ColorTargetState {
                format: TextureFormat::Bgra8UnormSrgb,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            })],
        }),
        multiview: None,
    };

    get_device().create_render_pipeline(&descriptor)
}

static mut RECTANGLE_PIPELINE: Option<RenderPipeline> = None;

fn get_rectangle_pipeline() -> &'static RenderPipeline {
    unsafe { RECTANGLE_PIPELINE.get_or_insert_with(create_rectangle_pipeline) }
}
