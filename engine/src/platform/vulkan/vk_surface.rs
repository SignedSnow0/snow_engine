use crate::platform::vulkan::vk_core::VkCore;
use crate::platform::window::Window;
use anyhow::Result;
use std::sync::Arc;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::swapchain::{Swapchain, SwapchainCreateInfo};

pub struct VkSurface {
    window: Window,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
}

impl VkSurface {
    pub fn new(window: Window, core: &VkCore) -> Result<VkSurface> {
        let handle = vulkano_win::create_surface_from_winit(
            window.handle().clone(),
            core.instance().clone(),
        )
        .expect("Failed to create surface");

        let capabilities = core.query_surface_capabilities(&handle)?;

        let composite_alpha = capabilities
            .supported_composite_alpha
            .iter()
            .next()
            .unwrap();

        let image_format = Some(core.query_surface_formats(&handle)?[0].0);

        let (swapchain, images) = Swapchain::new(
            core.device().clone(),
            handle.clone(),
            SwapchainCreateInfo {
                min_image_count: capabilities.min_image_count,
                image_format,
                image_extent: [window.size().0, window.size().1],
                image_usage: ImageUsage {
                    color_attachment: true,
                    ..ImageUsage::default()
                },
                composite_alpha,
                ..SwapchainCreateInfo::default()
            },
        )?;

        Ok(VkSurface {
            window,
            swapchain,
            images,
        })
    }

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }
}
