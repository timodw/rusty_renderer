use crate::renderer::Renderer;
use crate::world;
use crate::canvas;
use crate::world::vector3d::Vector3D;

pub struct Raytracer {
}

impl Renderer for Raytracer {
    fn render(&self, world: &world::World, canvas: &mut canvas::Canvas) {
        for x in -canvas.height / 2..canvas.height / 2 {
            for y in -canvas.width / 2..canvas.width / 2 {
                let (vp_x, vp_y) = canvas.to_viewport_coordinates(x, y, &world.viewport);
                let viewport_vector = Vector3D {
                    x: vp_x,
                    y: vp_y,
                    ..world.viewport.center
                };
                let ray_dir = viewport_vector - world.camera.position;
                println!("Camera: {:?} \nVP: {:?} \nD: {:?}", world.camera.position, viewport_vector, ray_dir);
            }
        }
    }
}