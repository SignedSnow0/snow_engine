use crate::platform::window::Window;
use anyhow::Result;
use std::default::Default;
use std::sync::Arc;
use vulkano::device::physical::{PhysicalDeviceError, PhysicalDeviceType};
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo,
};
use vulkano::format::Format;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::swapchain::{ColorSpace, Surface, SurfaceCapabilities, SurfaceInfo};
use vulkano::VulkanLibrary;

pub struct VkCore {
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl VkCore {
    pub fn new(window: &Window) -> Result<VkCore> {
        let library = VulkanLibrary::new()?;

        let required_extensions = vulkano_win::required_extensions(&library);

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                ..InstanceCreateInfo::default()
            },
        )?;

        let enabled_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let surface =
            vulkano_win::create_surface_from_winit(window.handle().clone(), instance.clone())
                .expect("Failed to create surface");

        let (physical_device, queue_index) =
            Self::choose_physical_device(&instance, &enabled_extensions, surface)?;

        let (device, mut queue) =
            Self::create_device(physical_device, enabled_extensions, queue_index)?;

        let queue = queue.next().unwrap();

        Ok(VkCore {
            instance,
            device,
            queue,
        })
    }

    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn query_surface_capabilities(
        &self,
        handle: &Arc<Surface>,
    ) -> Result<SurfaceCapabilities, PhysicalDeviceError> {
        self.device
            .physical_device()
            .surface_capabilities(&handle, SurfaceInfo::default())
    }

    pub fn query_surface_formats(
        &self,
        handle: &Arc<Surface>,
    ) -> Result<Vec<(Format, ColorSpace)>, PhysicalDeviceError> {
        self.device
            .physical_device()
            .surface_formats(&handle, SurfaceInfo::default())
    }

    pub fn query_physical_devices(&self) {
        for physical_device in self.instance.enumerate_physical_devices().unwrap() {
            println!("Device: {}", physical_device.properties().device_name);
        }
    }

    fn choose_physical_device(
        instance: &Arc<Instance>,
        enabled_extensions: &DeviceExtensions,
        surface: Arc<Surface>,
    ) -> Result<(Arc<vulkano::device::physical::PhysicalDevice>, u32)> {
        let (physical_device, queue_index) = instance
            .enumerate_physical_devices()?
            .filter(|device| device.supported_extensions().contains(enabled_extensions))
            .filter_map(|device| {
                device
                    .queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(index, queue)| {
                        queue.queue_flags.graphics
                            && device
                                .surface_support(index as u32, &surface)
                                .unwrap_or(false)
                    })
                    .map(|queue| (device, queue as u32))
            })
            .max_by_key(|(device, _)| match device.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 4,
                PhysicalDeviceType::IntegratedGpu => 3,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 1,
                _ => 0,
            })
            .expect("No supported device found");

        Ok((physical_device, queue_index))
    }

    fn create_device(
        physical_device: Arc<vulkano::device::physical::PhysicalDevice>,
        enabled_extensions: DeviceExtensions,
        queue_index: u32,
    ) -> Result<(Arc<Device>, impl ExactSizeIterator<Item = Arc<Queue>>)> {
        let (device, queue) = {
            let enabled_features = Features::empty();

            Device::new(
                physical_device,
                DeviceCreateInfo {
                    enabled_extensions,
                    enabled_features,
                    queue_create_infos: vec![QueueCreateInfo {
                        queue_family_index: queue_index,
                        ..QueueCreateInfo::default()
                    }],
                    ..DeviceCreateInfo::default()
                },
            )?
        };

        Ok((device, queue))
    }
}
