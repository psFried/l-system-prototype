pub mod crab;

pub trait Renderer {
    fn forward(&mut self);

    fn left(&mut self);

    fn right(&mut self);
}
