use super::*;
use crate::font::Font;
use crate::gfx::*;
use drawing::{Drawable, UniformBuffer};
use font::FontSize;
use freetype::glyph;
use glm::{Mat4, Vec4};
use lazy_static::lazy_static;
use {wgpu, Color};
use {Point, Size};

#[repr(packed)]
#[allow(dead_code)]
struct GlyphUniform {
    transform: Mat4,
    color: Color,
}

struct Glyph {
    buffer: UniformBuffer<GlyphUniform>,
    bind_group: wgpu::BindGroup,
}

impl Glyph {
    fn new(rect: Rect, texture_view: &wgpu::TextureView) -> Self {
        let device = get_device();
        let buffer = UniformBuffer::new_initialized(GlyphUniform {
            transform: model_projection(rect, 0.0),
            color: Color::BLACK,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &GLYPH_INFO_BIND_LAYOUT,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: buffer.as_ref(),
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
            ],
        });

        Self { buffer, bind_group }
    }
}

impl Drawable for Glyph {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // note: pipeline bound by FormattedText
        // render_pass.set_pipeline(get_glyph_pipeline());
        render_pass.set_bind_group(1, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}

pub struct FormattedText {
    text: Box<str>,
    font: &'static Font,
    size: ::font::FontSize,
    rect: Rect,
    glyphs: Vec<Glyph>,
}

impl FormattedText {
    // todo: implement text alignment
    pub fn new(text: &str, rect: Rect, font: &'static Font, size: FontSize) -> Self {
        let mut glyphs = Vec::new();

        let font_scale = ::font::calculate_font_scale(size.as_pt());
        let line_height = font.line_height() * font_scale;
        let accent = font.ascent() * font_scale;
        //let decent = font.descent() * font_scale;

        let mut pos = rect.pos;
        pos.y = pos.y - line_height + accent;

        for line in text.lines() {
            for c in line.chars() {
                if c.is_control() {
                    continue;
                }

                let glyph = font.get_glyph(c).unwrap();
                if let Some(texture_view) = glyph.texture_view() {
                    let size = glyph.size() * font_scale;
                    let mut off = glyph.bearing() * font_scale;
                    off.y = -off.y;

                    let tex_scale = glyph.texture_size() / glyph.size();
                    let tex_off = -(size * tex_scale - size) / 2.0;

                    let p = pos + off + tex_off;
                    let s = size * tex_scale;

                    let mut rect = Rect::new(p, s);
                    rect.pos.y = rect.pos.y + line_height;

                    glyphs.push(Glyph::new(rect, texture_view));
                }
                pos.x += glyph.advance() * font_scale;
            }
            pos.x = rect.pos.x;
            pos.y += line_height;
        }

        Self {
            text: text.into(),
            font,
            size,
            rect,
            glyphs,
        }
    }

    pub fn update_pos(&mut self, origin: Point) {
        todo!()
    }

    pub fn update_text(&mut self, text: &str) {
        todo!()
    }
}

impl Drawable for FormattedText {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&GLYPH_RENDER_PIPELINE);
        for glyph in &self.glyphs {
            glyph.draw(render_pass);
        }
    }
}

lazy_static! {
    static ref GLYPH_VERTEX_SHADER: wgpu::ShaderModule = {
        get_device().create_shader_module(include_spirv!(concat!(
            env!("OUT_DIR"),
            "/shaders/glyph_sdf.vert.spv"
        )))
    };
    static ref GLYPH_FRAGMENT_SHADER: wgpu::ShaderModule = {
        get_device().create_shader_module(include_spirv!(concat!(
            env!("OUT_DIR"),
            "/shaders/glyph_sdf.frag.spv"
        )))
    };
    static ref GLYPH_INFO_BIND_LAYOUT: wgpu::BindGroupLayout = {
        get_device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Glyph Info Binding Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            std::num::NonZeroU64::new(std::mem::size_of::<GlyphUniform>() as u64)
                                .unwrap(),
                        ),
                    },
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
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
    static ref GLYPH_PIPELINE_LAYOUT: wgpu::PipelineLayout = {
        get_device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Glyph Pipeline Layout"),
            bind_group_layouts: &[&RENDER_INFO_BIND_LAYOUT, &GLYPH_INFO_BIND_LAYOUT],
            push_constant_ranges: &[],
        })
    };
    static ref GLYPH_RENDER_PIPELINE: wgpu::RenderPipeline = {
        get_device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Glyph Render Pipeline"),
            layout: Some(&GLYPH_PIPELINE_LAYOUT),
            vertex: wgpu::VertexState {
                module: &GLYPH_VERTEX_SHADER,
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
                module: &GLYPH_FRAGMENT_SHADER,
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
