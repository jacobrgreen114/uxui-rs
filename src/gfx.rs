use wgpu::*;

static mut INSTANCE: Option<Instance> = None;
static mut ADAPTER: Option<Adapter> = None;
static mut DEVICE_QUEUE: Option<(Device, Queue)> = None;

pub(crate) fn get_instance() -> &'static Instance {
    unsafe { INSTANCE.get_or_insert_with(create_instance) }
}

pub(crate) fn get_adapter() -> &'static Adapter {
    unsafe { ADAPTER.get_or_insert_with(create_adapter) }
}

pub(crate) fn get_device() -> &'static Device {
    unsafe { &DEVICE_QUEUE.get_or_insert_with(create_device_queue).0 }
}

pub(crate) fn get_queue() -> &'static Queue {
    unsafe { &DEVICE_QUEUE.get_or_insert_with(create_device_queue).1 }
}

fn create_instance() -> Instance {
    Instance::new(InstanceDescriptor::default())
}

fn create_adapter() -> Adapter {
    futures::executor::block_on(get_instance().request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::None,
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .unwrap()
}

fn create_device_queue() -> (Device, Queue) {
    futures::executor::block_on(get_adapter().request_device(
        &DeviceDescriptor {
            label: Some("Uxui device"),
            features: Features::default(),
            limits: Limits::default(),
        },
        None,
    ))
    .unwrap()
}

pub(crate) fn single_submit(f: impl FnOnce(&mut CommandEncoder)) {
    let device = get_device();
    let queue = get_queue();
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Uxui command encoder"),
    });
    f(&mut encoder);
    queue.submit(Some(encoder.finish()));
}
