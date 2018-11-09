use super::Renderer;
use turtle::Turtle;

pub struct Crab {
    step: f64,
    angle: f64,
    turtle: Turtle,
}

impl Crab {
    pub fn new<C>(c: C, turtle: Turtle) -> Self
    where
        C: Into<Config>,
    {
        let config = c.into();
        Self {
            step: config.step,
            angle: config.angle,
            turtle,
        }
    }
}

impl Renderer for Crab {
    fn forward(&mut self) {
        self.turtle.forward(self.step);
    }

    fn left(&mut self) {
        self.turtle.left(self.angle);
    }

    fn right(&mut self) {
        self.turtle.right(self.angle);
    }
}

pub struct Config {
    step: f64,
    angle: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            step: 100.0,
            angle: 60.0,
        }
    }
}

impl From<(f64, f64)> for Config {
    fn from(tuple: (f64, f64)) -> Self {
        Self {
            step: tuple.0,
            angle: tuple.1,
        }
    }
}
