use vulkano::{instance::Instance, swapchain::Surface};
use winit::{event_loop::{self, EventLoop}, window::WindowBuilder};
use std::sync::Arc;
use anyhow::Result;

use crate::{CoreSystem, graphics::GraphicsSystem};

pub struct Window {
    handle: Arc<winit::window::Window>,
    pub surface: Arc<Surface>
}

impl Window {
    pub fn create(graphics: &GraphicsSystem) -> Result<Window> {
        let handle = Arc::new(WindowBuilder::new()
            .build(&graphics.event_loop)?);

        let surface = Surface::from_window(graphics.instance, handle)?;

        Ok(Window { handle, surface }) 
    }

    pub(in graphics) fn create_private(event_loop: &Arc<EventLoop<()>>, instance: &Arc<Instance>) -> Result<Self> {
        let handle = Arc::new(WindowBuilder::new()
            .build(event_loop)?);

        let surface = Surface::from_window(instance.clone(), handle)?;

        Ok(Window { handle, surface })
    }
}

