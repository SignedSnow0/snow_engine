mod graphics;

use winit::event::{Event, WindowEvent};

use crate::graphics::{GraphicsSystem, render_pass::RenderPass};

fn main() {
    let (graphics, window) = GraphicsSystem::create().unwrap();
    let render_pass = RenderPass::create_from_window(&graphics, window).unwrap();

    graphics.event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { 
            event: WindowEvent::CloseRequested,
            ..
        } => {
            elwt.exit();
        },
        Event::WindowEvent { 
            event: WindowEvent::Resized(_),
            ..
        } => {
            let clear_values = vec![Some([0.0, 0.0, 1.0, 1.0].into())];
            let (window, mut command_buffer) = window.acquire_next_image(&graphics).unwrap();
            let command_buffer = command_buffer.begin_render_pass(&render_pass, clear_values).unwrap();

            let command_buffer = command_buffer.end_render_pass().unwrap();
            window.submit_command_buffer(command_buffer).unwrap();
            
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
        },
        _ => {}
    });
}
