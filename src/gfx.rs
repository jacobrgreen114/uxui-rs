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

use lazy_static::lazy_static;
use wgpu::*;

lazy_static! {
    static ref INSTANCE: Instance = Instance::new(InstanceDescriptor {
        backends: Backends::VULKAN,
        ..Default::default()
    });
    static ref ADAPTER: Adapter =
        futures::executor::block_on(get_instance().request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface: None,
        }))
        .unwrap();
    static ref DEVICE_QUEUE: (Device, Queue) =
        futures::executor::block_on(get_adapter().request_device(
            &DeviceDescriptor {
                label: Some("Uxui device"),
                features: Features::default() | Features::SPIRV_SHADER_PASSTHROUGH,
                limits: Limits::default(),
            },
            None,
        ))
        .unwrap();
}

#[inline]
pub(crate) fn get_instance() -> &'static Instance {
    // unsafe { INSTANCE.get_or_insert_with(create_instance) }
    &INSTANCE
}

#[inline]
pub(crate) fn get_adapter() -> &'static Adapter {
    // unsafe { ADAPTER.get_or_insert_with(create_adapter) }
    &ADAPTER
}

#[inline]
pub(crate) fn get_device() -> &'static Device {
    // unsafe { &DEVICE_QUEUE.get_or_insert_with(create_device_queue).0 }
    &DEVICE_QUEUE.0
}

#[inline]
pub(crate) fn get_queue() -> &'static Queue {
    // unsafe { &DEVICE_QUEUE.get_or_insert_with(create_device_queue).1 }
    &DEVICE_QUEUE.1
}

// pub(crate) fn single_submit(f: impl FnOnce(&mut CommandEncoder)) {
//     let device = get_device();
//     let queue = get_queue();
//     let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
//         label: Some("Uxui command encoder"),
//     });
//     f(&mut encoder);
//     queue.submit(Some(encoder.finish()));
// }

pub(crate) fn initialize() {
    lazy_static::initialize(&DEVICE_QUEUE);
}
