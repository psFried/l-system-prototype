pub mod crab;
pub mod string;
pub mod collection;

pub trait Renderer {
    fn forward(&mut self);

    fn left(&mut self);

    fn right(&mut self);
}
