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
use crate::drawing::*;
use crate::*;

#[repr(packed)]
#[derive(Debug)]
struct RectangleUniform {
    transform: Mat4,
    color: Vec4,
}

#[derive(Debug)]
pub struct VisualRectangle {
    buffer: UniformBuffer<RectangleUniform>,
    bind_group: wgpu::BindGroup,
}

impl VisualRectangle {
    pub fn new(rect: Rect, color: ::Color) -> Self {
        let device = get_device();

        let buffer = UniformBuffer::new_initialized(RectangleUniform {
            transform: model_projection(rect, 0.0),
            color: color.into(),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Rectangle Uniform Bind Group"),
            layout: &RECT_BIND_GROUP_LAYOUT,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.as_ref().as_entire_buffer_binding()),
            }],
        });

        Self { buffer, bind_group }
    }

    pub fn update(&self, rect: Rect, color: ::Color) {
        self.buffer.update(&RectangleUniform {
            transform: model_projection(rect, 0.0),
            color: color.into(),
        });
    }
}

impl Visual for VisualRectangle {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&RECT_PIPELINE);
        render_pass.set_bind_group(1, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}

lazy_static! {
    static ref RECT_VERTEX_SHADER: wgpu::ShaderModule = unsafe {
        get_device().create_shader_module_spirv(&include_spirv_raw!(concat!(
            env!("OUT_DIR"),
            "/shaders/rect.vert.spv"
        )))
    };
    static ref RECT_FRAGMENT_SHADER: wgpu::ShaderModule = unsafe {
        get_device().create_shader_module_spirv(&include_spirv_raw!(concat!(
            env!("OUT_DIR"),
            "/shaders/rect.frag.spv"
        )))
    };
    static ref RECT_BIND_GROUP_LAYOUT: wgpu::BindGroupLayout = {
        get_device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Rectangle Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                count: None,
            }],
        })
    };
    static ref RECT_PIPELINE_LAYOUT: wgpu::PipelineLayout = {
        get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Rectangle Pipeline Layout"),
            bind_group_layouts: &[&RENDER_INFO_BIND_LAYOUT, &RECT_BIND_GROUP_LAYOUT],
            push_constant_ranges: &[],
        })
    };
    static ref RECT_PIPELINE: wgpu::RenderPipeline = {
        get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rectangle Pipeline"),
            layout: Some(&RECT_PIPELINE_LAYOUT),
            vertex: wgpu::VertexState {
                module: &RECT_VERTEX_SHADER,
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
                module: &RECT_FRAGMENT_SHADER,
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
