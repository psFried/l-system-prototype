extern crate turtle;

use std::env;
use std::collections::HashMap;
use turtle::Turtle;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args
        .get(1)
        .unwrap_or(&String::from("1"))
        .parse::<i32>()
        .expect("enter a valid number as first argument");

    let mut rules: Rules = HashMap::new();
    rules.insert(
        Variable::F,
        vec![
            Variable::F,
            Variable::Minus,
            Variable::F,
            Variable::Plus,
            Variable::Plus,
            Variable::F,
            Variable::Minus,
            Variable::F,
        ],
    );

    let mut word = vec![
        Variable::F,
    ];

    for _ in 0..n {
        word = apply(&rules, word);
    }

    let mut turtle = Turtle::new();
    turtle.set_heading(0.0);

    let config = (400.0 / (3.0f64).powi(n), 60.0);
    draw(&word, &mut turtle, config);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Variable {
    F,
    Plus,
    Minus,
}

type Word = Vec<Variable>;

type Rules = HashMap<Variable, Vec<Variable>>;

fn apply(rules: &Rules, word: Word) -> Word {
    word
        .into_iter()
        .fold(Vec::new(), |mut acc, variable|{
            match rules.get(&variable) {
                Some(substitution) => {
                    for var in substitution {
                        acc.push(var.clone());
                    }
                }

                None => {
                    acc.push(variable)
                }
            }
            acc
        })
}

fn draw<C>(word: &Word, turtle: &mut Turtle, c: C)
where
    C: Into<Config>,
{
    let config: Config = c.into();
    for variable in word {
        match variable {
            Variable::F => {
                turtle.forward(config.step);
            }

            Variable::Minus => {
                turtle.left(config.angle);
            }

            Variable::Plus => {
                turtle.right(config.angle);
            }
        }
    }
}

struct Config {
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
