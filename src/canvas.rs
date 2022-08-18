use image;

use crate::world::Viewport;

#[derive(Debug)]
pub struct Canvas {
    pub width: i64,
    pub height: i64,
    pub grid: Vec<Vec<Pixel>>
}

impl Canvas {
    pub fn init(width: i64, height: i64) -> Canvas {
        Canvas {
            width: width,
            height: height,
            grid: vec![vec![Pixel { r: 128, g: 128, b: 0 }; width as usize]; height as usize]
        }
    }

    pub fn write_to_file(&self, path: &str) {
        let img = self.to_image();
        img.save(path).expect("ERROR: Something went wrong whilst saving the image!");
    }

    fn to_image(&self) -> image::RgbImage {
        let img = image::RgbImage::from_fn(
            self.width as u32,
            self.height as u32,
            |x, y| {
                let r: u8 = (&self.grid[x as usize])[y as usize].r;
                let g: u8 = (&self.grid[x as usize])[y as usize].g;
                let b: u8 = (&self.grid[x as usize])[y as usize].b;
                image::Rgb([r, g, b])
            }
        );

        img
    }

    pub fn to_viewport_coordinates(&self, c_x: i64, c_y: i64, vp: &Viewport) -> (f64, f64) {
        let v_x: f64 = (c_x as f64) * (vp.width / self.width as f64);
        let v_y: f64 = (c_y as f64) * (vp.height / self.height as f64);
        (v_x, v_y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

impl Pixel {
    fn add_intensity(& mut self, i: i8) {
        self.r = std::cmp::max(std::cmp::min(0, self.r as i8 + i) as u8, 255);
    }
}