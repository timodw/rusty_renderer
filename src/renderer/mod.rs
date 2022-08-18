pub mod raytracer;

use crate::canvas::Canvas;
use crate::world::World;

pub trait Renderer {
    fn render(&self, world: &World, canvas: &mut Canvas);
}