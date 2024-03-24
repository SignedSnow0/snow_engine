use anyhow::Result;
use vulkano::{device::Device, format::Format, image::{view::ImageView, Image}, pipeline::graphics::viewport::Viewport, render_pass::{Framebuffer, FramebufferCreateInfo}};
use std::sync::Arc;
use crate::graphics::{window::Window, GraphicsSystem};

pub struct RenderPass {
    pub render_pass: Arc<vulkano::render_pass::RenderPass>,
    window: Option<Arc<Window>>,
    viewport: Viewport,
    pub framebuffers: Vec<Arc<Framebuffer>>,
}

impl RenderPass {
    pub fn create_from_window(core: &GraphicsSystem, window: Arc<Window>) -> Result<Self> {
        let render_pass = create_render_pass(&core.device, window.swapchain.image_format())?;

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [window.swapchain.image_extent()[0] as f32, window.swapchain.image_extent()[1] as f32],
            depth_range: 0.0..=1.0,
        };

        let framebuffers = create_framebuffers(&window.images, &render_pass, &viewport)?; 

        Ok(RenderPass { render_pass, window: Some(window), viewport, framebuffers })
    }

    pub fn get_framebuffer(&self, index: usize) -> Result<Arc<Framebuffer>> {
        Ok(self.framebuffers[index].try_into().clone()?)
    }
}

fn create_render_pass(device: &Arc<Device>, format: Format) -> Result<Arc<vulkano::render_pass::RenderPass>> {
    Ok(vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                format: format,
                samples: 1,
                load_op: Clear,
                store_op: Store,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    ).unwrap())
}

fn create_framebuffers(images: &Vec<Arc<Image>>, render_pass: &Arc<vulkano::render_pass::RenderPass>, viewport: &Viewport) -> Result<Vec<Arc<Framebuffer>>> {
    Ok(images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                }
            )
            .unwrap()
        })
    .collect::<Vec<_>>())
}
