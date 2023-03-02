use snow_engine::platform::vulkan::vk_core::VkCore;
use snow_engine::platform::vulkan::vk_surface::VkSurface;
use snow_engine::platform::window::Window;

fn main() {
    let window = Window::new("Snow Engine", (1240, 720), Some(false), None)
        .expect("Failed to create window");
    let core = VkCore::new(&window).expect("Failed to create vulkan core");
    let mut surface = VkSurface::new(window, &core).expect("Failed to create surface");

    loop {
        surface.window().update();

        if surface.window().closing() {
            break;
        }
    }
}
