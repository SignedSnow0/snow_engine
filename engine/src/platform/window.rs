use anyhow::Result;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

pub struct Window {
    handle: Arc<winit::window::Window>,
    event_loop: EventLoop<()>,
    closing: bool,
}

impl Window {
    pub fn new(
        title: &str,
        size: (u32, u32),
        resizable: Option<bool>,
        visible: Option<bool>,
    ) -> Result<Window> {
        let event_loop = EventLoop::new();
        let handle = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(size.0, size.1))
            .with_resizable(resizable.unwrap_or(true))
            .with_visible(visible.unwrap_or(true))
            .build(&event_loop)?;

        Ok(Window {
            handle: Arc::new(handle),
            event_loop,
            closing: false,
        })
    }

    pub fn closing(&self) -> bool {
        self.closing
    }

    pub fn size(&self) -> (u32, u32) {
        self.handle.inner_size().into()
    }
    
    pub fn handle(&self) -> &Arc<winit::window::Window> {
        &self.handle
    }

    pub fn update(&mut self) {
        self.event_loop
            .run_return(|event, _, control_flow| match event {
                Event::MainEventsCleared => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    self.closing = true;
                }
                _ => {}
            });
    }
}
