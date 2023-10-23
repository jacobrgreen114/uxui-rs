use super::*;

use lazy_static::lazy_static;

struct ImageUniform {
    transform: Mat4,
}

pub struct VisualImage {
    uniform: UniformBuffer<ImageUniform>,
    bind_group: wgpu::BindGroup,
}

impl VisualImage {
    pub fn new(rect: Rect, texture_view: &wgpu::TextureView) -> Self {
        let uniform = UniformBuffer::new_initialized(ImageUniform {
            transform: model_projection(rect, 0.0),
        });

        let bind_group = get_device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &IMAGE_BIND_GROUP_LAYOUT,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        uniform.as_ref().as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
            ],
        });

        Self {
            uniform,
            bind_group,
        }
    }
}

impl Visual for VisualImage {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // render_pass.set_pipeline(&IMAGE_PIPELINE);
        // render_pass.set_bind_group(1, &self.bind_group, &[]);
        // render_pass.draw(0..6, 0..1);

        draw_quad(
            render_pass,
            &IMAGE_PIPELINE,
            &[BindGroupBinding {
                index: 1,
                bind_group: &self.bind_group,
            }],
        )
    }
}

struct BindGroupBinding<'a> {
    index: u32,
    bind_group: &'a wgpu::BindGroup,
}

#[inline(always)]
fn draw_quad<'a>(
    render_pass: &mut wgpu::RenderPass<'a>,
    pipeline: &'a wgpu::RenderPipeline,
    bind_groups: &[BindGroupBinding<'a>],
) {
    render_pass.set_pipeline(pipeline);
    bind_groups.iter().for_each(|bind_group| {
        render_pass.set_bind_group(bind_group.index, bind_group.bind_group, &[]);
    });
    render_pass.draw(0..6, 0..1);
}

lazy_static! {
    static ref IMAGE_VERTEX_SHADER: wgpu::ShaderModule = {
        get_device().create_shader_module(include_spirv!(concat!(
            env!("OUT_DIR"),
            "/shaders/image.vert.spv"
        )))
    };
    static ref IMAGE_FRAGMENT_SHADER: wgpu::ShaderModule = {
        get_device().create_shader_module(include_spirv!(concat!(
            env!("OUT_DIR"),
            "/shaders/image.frag.spv"
        )))
    };
    static ref IMAGE_BIND_GROUP_LAYOUT: wgpu::BindGroupLayout = {
        get_device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Image Binding Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    visibility: wgpu::ShaderStages::FRAGMENT,
                },
            ],
        })
    };
    static ref IMAGE_PIPELINE_LAYOUT: wgpu::PipelineLayout = {
        get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Image Pipeline Layout"),
            bind_group_layouts: &[&RENDER_INFO_BIND_LAYOUT, &IMAGE_BIND_GROUP_LAYOUT],
            push_constant_ranges: &[],
        })
    };
    static ref IMAGE_PIPELINE: wgpu::RenderPipeline = {
        get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Image Render Pipeline"),
            layout: Some(&IMAGE_PIPELINE_LAYOUT),
            vertex: wgpu::VertexState {
                module: &IMAGE_VERTEX_SHADER,
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
                module: &IMAGE_FRAGMENT_SHADER,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        })
    };
}
