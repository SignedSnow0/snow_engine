mod graphics;

use crate::graphics::GraphicsSystem;

fn main() {
    let (graphics, window) = GraphicsSystem::create().unwrap();

}
