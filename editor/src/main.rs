use snow_engine::platform::window::Window;

fn main() {
    let mut window = Window::new("Snow Engine", (1240, 720), None, None).expect("Failed to create window");

    loop {
        window.update();
        
        if window.closing() {
            break;
        }
    }
}
