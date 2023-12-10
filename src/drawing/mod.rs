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

mod image;
mod rect;
mod text;

pub use self::image::*;
pub use self::rect::*;
pub use self::text::*;

use crate::gfx::*;

use std::mem::size_of;
use std::ops::{Deref, DerefMut};

use glm::ext::*;
use glm::*;
use num_traits::identities::One;

use wgpu;
use wgpu::{include_spirv, include_spirv_raw, BufferSize, ShaderSource};
use Rect;

use lazy_static::lazy_static;

pub trait Visual {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub struct DrawingContext<'a> {
    render_pass: wgpu::RenderPass<'a>,
    scissor_stack: Vec<Rect>,
}

impl<'a> DrawingContext<'a> {
    pub(crate) fn new(render_pass: wgpu::RenderPass<'a>, initial_scissor: Rect) -> Self {
        Self {
            render_pass,
            scissor_stack: vec![initial_scissor],
        }
    }

    #[inline]
    pub fn draw<D>(&mut self, drawable: &'a D)
    where
        D: Visual,
    {
        drawable.draw(&mut self.render_pass)
    }

    pub fn push_scissor(&mut self, rect: Rect) {
        self.render_pass.set_scissor_rect(
            rect.pos.x as u32,
            rect.pos.y as u32,
            rect.size.width as u32,
            rect.size.height as u32,
        );
        self.scissor_stack.push(rect);
    }

    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
        let scissor = self.scissor_stack.last().unwrap();
        self.render_pass.set_scissor_rect(
            scissor.pos.x as u32,
            scissor.pos.y as u32,
            scissor.size.width as u32,
            scissor.size.height as u32,
        );
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

#[derive(Debug)]
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

    pub fn update(&self, data: &T) {
        get_queue().write_buffer(&self.buffer, 0, unsafe { to_slice(data) });
    }
}

unsafe fn to_slice<T>(data: &T) -> &[u8] {
    std::slice::from_raw_parts(data as *const T as *const u8, size_of::<T>())
}

impl<T> AsRef<wgpu::Buffer> for Buffer<T>
where
    T: Sized,
{
    fn as_ref(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

#[derive(Debug)]
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

    pub fn update(&self, data: &T) {
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

lazy_static! {
    pub(crate) static ref RENDER_INFO_BIND_LAYOUT: wgpu::BindGroupLayout = get_device()
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Global Binding Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
    pub(crate) static ref GLOBAL_SAMPLER: wgpu::Sampler =
        get_device().create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Global Sampler"),
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });
}
