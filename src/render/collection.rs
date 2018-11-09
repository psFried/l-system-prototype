use super::Renderer;
use std::mem;

pub fn batch<'a>(renderers: Vec<&'a mut dyn Renderer>) -> Collection<'a> {
    Collection::new(renderers)
}

pub struct Collection<'a> {
    renderers: Vec<&'a mut dyn Renderer>
}

impl<'a> Collection<'a> {
    pub fn new(renderers: Vec<&'a mut dyn Renderer>) -> Self {
        Self { renderers }
    }
}

impl<'a> Renderer for Collection<'a> {
    fn forward(&mut self) {
        let mut renderers = mem::replace(&mut self.renderers, vec![]);
        for ref mut renderer in &mut renderers {
            renderer.forward();
        }
        self.renderers = renderers;
    }

    fn left(&mut self) {
        let mut renderers = mem::replace(&mut self.renderers, vec![]);
        for ref mut renderer in &mut renderers {
            renderer.left();
        }
        self.renderers = renderers;
    }

    fn right(&mut self) {
        let mut renderers = mem::replace(&mut self.renderers, vec![]);
        for ref mut renderer in &mut renderers {
            renderer.right();
        }
        self.renderers = renderers;
    }
}
