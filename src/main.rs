extern crate turtle;

use turtle::Turtle;

fn main() {
    let config = (100.0, 60.0);
    let word = vec![
        Variable::F,
        Variable::Minus,
        Variable::F,
        Variable::Plus,
        Variable::Plus,
        Variable::F,
        Variable::Minus,
        Variable::F
    ];

    let mut turtle = Turtle::new();
    turtle.set_heading(0.0);

    draw(&word, &mut turtle, config);
}

enum Variable {
    F,
    Plus,
    Minus,
}

type Word = Vec<Variable>;


fn draw<C>(word: &Word, turtle: &mut Turtle, c: C)
where C: Into<Config> {
    let config: Config = c.into();
    for variable in word {
        match variable {
            Variable::F => { turtle.forward(config.step); }

            Variable::Minus => { turtle.left(config.angle); }

            Variable::Plus => { turtle.right(config.angle); }
        }
    }
}

struct Config {
    step: f64,
    angle: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self { step: 100.0, angle: 60.0 }
    }
}

impl From<(f64, f64)> for Config {
    fn from(tuple: (f64, f64)) -> Self {
        Self { step: tuple.0, angle: tuple.1 }
    }
}
