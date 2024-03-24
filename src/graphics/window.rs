use vulkano::{device::{Device, Queue}, image::Image, instance::Instance, swapchain::{self, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainPresentInfo}, sync::{self, GpuFuture}, Validated, ValidationError, VulkanError};
use winit::{event_loop::EventLoop, window::WindowBuilder};
use std::sync::Arc;
use anyhow::Result;

use crate::graphics::{command_buffer::CommandBuffer, GraphicsSystem};

pub struct Window {
    pub surface: Arc<Surface>,
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<Image>>,
    pub queue: Arc<Queue>,
    acquire_future: Option<SwapchainAcquireFuture>,
    previous_frame_end: Box<dyn GpuFuture>
}

impl Window {
    /// Creates a new window.
    /// graphics: The graphics system to create the window with.
    pub fn create(graphics: &GraphicsSystem) -> Result<Arc<Window>> {
        let surface = create_surface(&graphics.event_loop, &graphics.instance)?;

        Self::create_private(surface, &graphics.device, &graphics.queue) 
    }

    /// Creates a new window from the raw surface.
    /// Should be used only by the graphics system in initialization phase.
    pub(in crate::graphics) fn create_private(surface: Arc<Surface>, device: &Arc<Device>, queue: &Arc<Queue>) -> Result<Arc<Self>> {
        let (swapchain, images) = create_swapchain(&surface, device)?;
       
        let previous_frame_end = Box::new(vulkano::sync::now(device.clone())) as Box<dyn vulkano::sync::GpuFuture>;

        Ok(Arc::new(Window { surface, swapchain, images, queue: queue.clone(), acquire_future: None, previous_frame_end }))
    }

    pub fn acquire_next_image(&mut self, core: &GraphicsSystem) -> Result<(&mut Self, CommandBuffer)> {
        self.previous_frame_end.as_mut().cleanup_finished();
        
        let (image_index, suboptimal, acquire_future) = 
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(result) => result,
                Err(Validated::Error(VulkanError::OutOfDate)) => {
                    return Err(anyhow::anyhow!("Swapchain out of date"));
                },
                Err(err) => {
                    return Err(anyhow::anyhow!("Failed to acquire next image: {:?}", err));
                }
            };
        self.acquire_future = Some(acquire_future);
        
        let command_buffer = CommandBuffer::create(core, image_index.try_into()?)?;

        Ok((self, command_buffer))
    }

    pub fn submit_command_buffer(&mut self, command_buffer: CommandBuffer) -> Result<&mut Self> {
        if let Some(acquire_future) = self.acquire_future.take() {
            let cmd = command_buffer.builder.build()?;

            let future = self.previous_frame_end
                .join(acquire_future)
                .then_execute(self.queue.clone(), cmd)
                .unwrap()
                .then_swapchain_present(self.queue.clone(), SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), command_buffer.current_image as u32))
                .then_signal_fence_and_flush();
            
            match future {
                Ok(future) => {
                    self.previous_frame_end = Box::new(future) as Box<_>;
                },
                Err(e) => {
                    self.previous_frame_end = Box::new(sync::now(self.queue.device().clone())) as Box<_>;
                    return Err(anyhow::anyhow!("Failed to flush future: {:?}", e));
                }
            }
            
        } else {
            return Err(anyhow::anyhow!("No acquire future found"));
        }

        

        Ok(self)
    }
}

/// Helper function to create a surface.
/// Should be used only by the graphics system in initialization phase.
pub(in crate::graphics) fn create_surface(event_loop: &EventLoop<()>, instance: &Arc<Instance>) -> Result<Arc<Surface>> {
    let handle = Arc::new(WindowBuilder::new()
        .build(event_loop)?);

    let surface = Surface::from_window(instance.clone(), handle)?;

    Ok(surface)
}

/// Helper function used by the window to create a swapchain.
fn create_swapchain(surface: &Arc<Surface>, device: &Arc<Device>) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>)>{
    let capabilities = device.physical_device().surface_capabilities(surface, Default::default()).unwrap();

    let usage = capabilities.supported_usage_flags;
    let alpha = capabilities.supported_composite_alpha.into_iter().next().unwrap();

    let image_format = device.physical_device()
            .surface_formats(surface, Default::default())
            .unwrap()[0]
            .0;

    let window = surface.object().unwrap().downcast_ref::<winit::window::Window>().unwrap();
    let image_extent: [u32; 2] = window.inner_size().into();

    Ok(Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: capabilities.min_image_count,
            image_format,
            image_usage: usage,
            image_extent,
            composite_alpha: alpha,
            ..Default::default()
        },
    )?)
}
