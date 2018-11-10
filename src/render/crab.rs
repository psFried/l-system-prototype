use super::Renderer;
use turtle::{Turtle, Point, Angle};

pub struct Crab {
    step: f64,
    angle: f64,
    stack: Vec<State>,
    turtle: Turtle,
}

struct State {
    position: Point,
    heading: Angle,
}

impl State {
    fn new(position: Point, heading: Angle) -> Self {
        Self { position, heading }
    }
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
            stack: Vec::new(),
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

    fn push(&mut self) {
        let position = self.turtle.position();
        let heading = self.turtle.heading();
        let state = State::new(position, heading);
        self.stack.push(state);
    }

    fn pop(&mut self) {
        let state_option = self.stack.pop();
        if state_option.is_some() {
            let state = state_option.unwrap();
            self.turtle.pen_up();
            self.turtle.go_to(state.position);
            self.turtle.set_heading(state.heading);
            self.turtle.pen_down();
        }
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
