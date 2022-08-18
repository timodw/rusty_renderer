pub mod vector3d;

use vector3d::Vector3D;

#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub center: Vector3D,
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub position: Vector3D,
    pub direction: Vector3D
}

#[derive(Debug)]
pub struct World {
    pub camera: Camera,
    pub viewport: Viewport
}

impl World {
    pub fn init() -> World {
        World {
            camera: Camera { 
                position: Vector3D::init(0., 0., 0.),
                direction: Vector3D::init(0., 0., 1.)
            },
            viewport: Viewport { 
                width: 1.,
                height: 1.,
                center: Vector3D::init(0., 0., 1.)
            } }
    }
}