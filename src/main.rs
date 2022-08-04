#[derive(Debug)]
struct Pixel{
    r: u8,
    g: u8,
    b: u8
}

impl Pixel {
    fn add_intensity(& mut self, i: i8) {
        self.r = std::cmp::max(std::cmp::min(0, self.r as i8 + i) as u8, 255);
    }
}


#[derive(Debug)]
struct Canvas{
    width: u32,
    height: u32,
}


fn main() {
    let p_1 = Pixel { r: 123, g: 234, b: 56 };
    println!("Pixel: {:?}!", p_1);
}
