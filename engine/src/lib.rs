pub mod platform;

#[cfg(test)]
mod tests {
    use crate::platform::vulkan::vk_core::VkCore;
    use crate::platform::vulkan::vk_surface::VkSurface;
    use crate::platform::window::Window;

    #[test]
    fn create_vk_core() {
        let core = VkCore::new().unwrap();

        core.query_physical_devices();
    }
}
