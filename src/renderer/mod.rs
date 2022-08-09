use canvas::Canvas;
use object::Object;

pub trait Renderer {
    fn render(&self, obj: &Object) -> Canvas;
}