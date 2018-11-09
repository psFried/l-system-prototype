use super::Renderer;
use std::fmt;

pub struct Collector {
    collected: String,
}

impl Collector {
    pub fn new() -> Self {
        Self { collected : String::from("") }
    }
}

impl Renderer for Collector {
    fn forward(&mut self) {
        self.collected += "F";
    }

    fn left(&mut self) {
        self.collected += "-"
    }

    fn right(&mut self) {
        self.collected += "+"
    }
}


impl fmt::Display for Collector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.collected)
    }
}
