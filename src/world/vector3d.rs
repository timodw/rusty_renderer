use std::ops::{ Add, Sub, Mul };

#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3D {
    pub fn init(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn get_magnitude(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector3D { 
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z 
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3D { 
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z 
        } 
    }
}

impl Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}
