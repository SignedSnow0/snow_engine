mod window;

use vulkano::{device::{physical::{PhysicalDevice, PhysicalDeviceType}, Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags}, instance::{Instance, InstanceCreateInfo}, swapchain::Surface, Version, VulkanLibrary};
use winit::event_loop::EventLoop;
use std::sync::Arc;
use anyhow::Result;
use crate::graphics::window::Window;

pub struct GraphicsSystem {
    pub event_loop: Arc<EventLoop<()>>,
    pub instance: Arc<Instance>,
    pub device: Arc<Device>,
}

impl GraphicsSystem {
    pub fn create() -> Result<(Self, Window)> {
        let event_loop = create_event_loop()?;

        let instance = create_instance(&event_loop)?;

        let window = Window::create_private(&event_loop, &instance)?; 

        let (physical_device, queue_family_index) = choose_physical_device(&instance, DeviceExtensions::none(), &window.surface)?; 

        let graphics = GraphicsSystem{ event_loop, instance };

        Ok((graphics, window))
    }
}

fn create_event_loop() -> Result<Arc<EventLoop<()>>> {
    Ok(Arc::new(EventLoop::new()?))
}

fn create_instance(event_loop: &Arc<EventLoop<()>>) -> Result<Arc<Instance>> {
    let library = VulkanLibrary::new().unwrap();
    let extensions = Surface::required_extensions(event_loop);

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: extensions,
            max_api_version: Some(Version::V1_2),
            ..Default::default()
        },
    )?;

    Ok(instance)
}

fn choose_physical_device(instance: &Arc<Instance>, required_extensions: &DeviceExtensions, surface: &Surface) -> Result<(Arc<Device>, u32)>{
    let (device, family) = instance.enumerate_physical_devices()?
     .filter(|device| device.supported_extensions().contains(required_extensions))
        .filter_map(|device| {
            device.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(family, queue)| {
                    queue.queue_flags.contains(QueueFlags::GRAPHICS) && device.surface_support(family as u32, surface).unwrap_or(false)
                })
                .map(|family| (device, family as u32))
        })
        .max_by_key(|(queue, _)| {
            match queue.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 2,
                PhysicalDeviceType::IntegratedGpu => 1,
                _ => 0
            }
        })?;

    Ok((device, family))
}

fn create_device(physical: Arc<PhysicalDevice>, queue_family_index: u32, required_extensions: DeviceExtensions) -> Result<(Arc<Device>, Arc<Queue>)> {
    let (device, mut queues) = Device::new(
        physical, 
        DeviceCreateInfo {
            enabled_extensions: required_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
    })?;

    let queue = queues.next().unwrap();
    Ok((device, queue))
}
