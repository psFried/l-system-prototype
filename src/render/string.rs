use super::Renderer;
use std::fmt;
use std::io::{stdout, Stdout};
use std::io::Write;

pub struct Reporter {
    stdout: Stdout
}

impl Reporter {
    pub fn new() -> Self {
        Self { stdout : stdout() }
    }
}

impl Renderer for Reporter {
    fn forward(&mut self) {
        let mut handle = self.stdout.lock();
        handle.write(b"F").unwrap_or(0);
        handle.flush().unwrap_or(());
    }

    fn left(&mut self) {
        let mut handle = self.stdout.lock();
        handle.write(b"-").unwrap_or(0);
        handle.flush().unwrap_or(());
    }

    fn right(&mut self) {
        let mut handle = self.stdout.lock();
        handle.write(b"+").unwrap_or(0);
        handle.flush().unwrap_or(());
    }
}

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
        self.collected += "-";
    }

    fn right(&mut self) {
        self.collected += "+";
    }
}


impl fmt::Display for Collector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.collected)
    }
}
