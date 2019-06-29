
use api::{Renderer, RendererConfig};

use turtle::{Turtle, Point, Angle};

pub struct Crab {
    step: f64,
    step_multiplier: f64,
    angle: f64,
    stack: Vec<State>,
    turtle: Turtle,
}

struct State {
    position: Point,
    heading: Angle,
    step: f64,
}

impl State {
    fn new(position: Point, heading: Angle, step: f64) -> Self {
        Self { position, heading, step }
    }
}

impl Crab {
    pub fn new<C>(c: C) -> Self
    where
        C: Into<RendererConfig>,
    {

        let mut turtle = Turtle::new();
        turtle.set_heading(65.0);
        let config = c.into();
        Self {
            step: config.step,
            step_multiplier: config.step_multiplier,
            angle: config.angle,
            stack: Vec::new(),
            turtle,
        }
    }
}

impl Renderer for Crab {

    fn global_init() {
        turtle::start();
    }

    fn forward(&mut self) {
        self.turtle.forward(self.step);
    }

    fn rotate_left(&mut self) {
        self.turtle.left(self.angle);
    }

    fn rotate_right(&mut self) {
        self.turtle.right(self.angle);
    }
    fn push(&mut self) {
        let position = self.turtle.position();
        let heading = self.turtle.heading();
        let state = State::new(position, heading, self.step);
        self.stack.push(state);
        self.step = self.step / self.step_multiplier;
    }

    fn pop(&mut self) {
        if let Some(state) = self.stack.pop() {
            self.step = state.step;
            self.turtle.pen_up();
            self.turtle.go_to(state.position);
            self.turtle.set_heading(state.heading);
            self.turtle.pen_down();
        }
    }

    fn finish(&mut self) {
        self.turtle.hide();
    }

}



