use crate::world::Viewport;

pub mod canvas;
pub mod world;
pub mod renderer;

use renderer::Renderer;

fn main() {
    let mut canvas = canvas::Canvas::init(8, 8);
    let world = world::World::init();
    let rt: renderer::raytracer::Raytracer = renderer::raytracer::Raytracer { };
    rt.render(&world, &mut canvas);
}
