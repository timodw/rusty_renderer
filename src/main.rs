use crate::canvas::Canvas;

pub mod canvas;

fn main() {
    let canvas = canvas::Canvas::init(128, 128);
    canvas.write_to_file("out/test.png");
    //println!("{:?}", canvas);
}
