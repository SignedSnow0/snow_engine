use anyhow::{Ok, Result};
use vulkano::{command_buffer::{allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo}, device::Queue, format::ClearValue, swapchain::{PresentFuture, SwapchainAcquireFuture, SwapchainPresentInfo}, sync::{self, future::FenceSignalFuture, GpuFuture}};
use super::{render_pass::RenderPass, GraphicsSystem};

pub struct CommandBuffer {
    pub builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    pub current_image: usize,
}

impl CommandBuffer {
    //TODO: this in window class
    pub fn create(core: &GraphicsSystem, current_image: usize) -> Result<Self> {
        let allocator = StandardCommandBufferAllocator::new(core.device.clone(), Default::default());
        let builder = AutoCommandBufferBuilder::primary(
            &allocator,
            core.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit
        )?;

        Ok(CommandBuffer { builder, current_image })
    }

    /// Begins a render pass in the current command buffer.
    /// clear_values: The clear values for the render pass.
    pub fn begin_render_pass(mut self, render_pass: &RenderPass, clear_values: Vec<Option<ClearValue>>) -> Result<Self> {
        self.builder.begin_render_pass(
            RenderPassBeginInfo {
                clear_values,
                ..RenderPassBeginInfo::framebuffer(render_pass.get_framebuffer(self.current_image)?)
            },
            SubpassBeginInfo {
                contents: SubpassContents::Inline,
                ..Default::default()
            }
        )?;
       
        Ok(self)
    }

    /// Ends the last active render pass in the current command buffer.
    pub fn end_render_pass(mut self) -> Result<Self> {
        self.builder.end_render_pass(
            SubpassEndInfo {
                ..Default::default()
            }
        )?;
        
        Ok(self)
    }
}
