use lazy_static::lazy_static;
use std::time::Instant;
use wgpu::*;

lazy_static! {
    static ref INSTANCE: Instance = Instance::default();
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
                features: Features::default(),
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
