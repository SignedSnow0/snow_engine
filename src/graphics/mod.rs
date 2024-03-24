pub mod window;
pub mod command_buffer;
pub mod render_pass;

use vulkano::{device::{physical::{PhysicalDevice, PhysicalDeviceType}, Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags}, instance::{Instance, InstanceCreateInfo}, swapchain::Surface, Version, VulkanLibrary};
use winit::{event::{Event, WindowEvent}, event_loop::EventLoop};
use std::sync::Arc;
use anyhow::Result;
use crate::graphics::window::Window;

pub struct GraphicsSystem {
    pub event_loop: EventLoop<()>,
    pub instance: Arc<Instance>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>
}

impl GraphicsSystem {
    /// Creates a new graphics system.
    /// In order to run the system at least one window must be created.
    /// Returns a tuple containing the graphics system and the window created.
    pub fn create() -> Result<(Self, Arc<Window>)> {
        let event_loop = create_event_loop()?;

        let instance = create_instance(&event_loop)?;

        let surface = window::create_surface(&event_loop, &instance)?;

        let required_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = choose_physical_device(&instance, &required_extensions, &surface)?;

        let (device, queue) = create_device(physical_device, queue_family_index, required_extensions)?;

        let window = Window::create_private(surface, &device, &queue)?; 
        
        let graphics = GraphicsSystem{ event_loop, instance, device, queue };

        
        Ok((graphics, window))
    }
}

/// Helper function used by GraphicsSystem to create an event loop.
fn create_event_loop() -> Result<EventLoop<()>> {
    Ok(EventLoop::new()?)
}

/// Helper function used by GraphicsSystem to create an instance.
fn create_instance(event_loop: &EventLoop<()>) -> Result<Arc<Instance>> {
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

/// Helper function used by GraphicsSystem to choose a physical device and queue_family_index.
fn choose_physical_device(instance: &Arc<Instance>, required_extensions: &DeviceExtensions, surface: &Surface) -> Result<(Arc<PhysicalDevice>, u32)>{
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
        })
        .expect("No suitable device found");

    Ok((device, family))
}

/// Helper function used by GraphicsSystem to create a device and queue.
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
