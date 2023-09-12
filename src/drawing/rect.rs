use crate::drawing::*;
use crate::*;

#[repr(packed)]
#[allow(dead_code)]
struct RectangleUniform {
    transform: Mat4,
    color: Vec4,
}

#[allow(dead_code)]
pub struct Rectangle {
    buffer: UniformBuffer<RectangleUniform>,
    bind_group: wgpu::BindGroup,
}

impl Rectangle {
    pub fn new(rect: Rect, color: ::Color) -> Self {
        let device = get_device();

        let buffer = UniformBuffer::new_initialized(RectangleUniform {
            transform: model_projection(rect, 0.0),
            color: color.into(),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Rectangle Uniform Bind Group"),
            layout: get_uniform_binding_layout(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.as_ref().as_entire_buffer_binding()),
            }],
        });

        Self { buffer, bind_group }
    }

    pub fn update(&mut self, rect: Rect, color: ::Color) {
        self.buffer.update(RectangleUniform {
            transform: model_projection(rect, 0.0),
            color: color.into(),
        });
    }
}

impl Drawable for Rectangle {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(get_rectangle_pipeline());
        render_pass.set_bind_group(1, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
