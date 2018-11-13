extern crate prototype;
extern crate turtle;

use prototype::render::crab::Crab;
use prototype::render::collection::batch;
use prototype::render::string::{Reporter, Collector};
use prototype::render::Renderer;
use std::collections::HashMap;
use std::env;
use turtle::Turtle;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args
        .get(1)
        .unwrap_or(&String::from("1"))
        .parse::<i32>()
        .expect("enter a valid number as first argument");

    let mut rules: Rules = Rules::new();
    rules.insert(
        Variable::new('F'),
        vec![
            Variable::new('F'),
            Variable::new('F'),
        ],
    );
    rules.insert(
        Variable::new('X'),
        vec![
            Variable::new('F'),
            Variable::new('+'),
            Variable::new('['),
            Variable::new('['),
            Variable::new('X'),
            Variable::new(']'),
            Variable::new('-'),
            Variable::new('X'),
            Variable::new(']'),
            Variable::new('-'),
            Variable::new('F'),
            Variable::new('['),
            Variable::new('-'),
            Variable::new('F'),
            Variable::new('X'),
            Variable::new(']'),
            Variable::new('+'),
            Variable::new('X')
        ],
    );

    let mut word = vec![Variable::new('X')];

    for _ in 0..n {
        word = apply(&rules, word);
    }


    let mut collector = Collector::new();
    let mut turtle = Turtle::new();
    turtle.set_heading(65.0);
    let config = (200.0 / 2.5f64.powi(n), 25.0);
    let mut crab = Crab::new(config, turtle);

    {
        let mut reporter = Reporter::new();
        let mut renderer = batch(vec![
            &mut collector,
            &mut crab,
            &mut reporter,
        ]);

        render(&word, &mut renderer);
    }

    println!("\n{}", collector);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Variable {
    symbol: char,
}

impl Variable {
    pub fn new(symbol: char) -> Self {
        Self { symbol }
    }
}

type Word = Vec<Variable>;

struct Rules {
    substitutions: HashMap<Variable, Vec<Variable>>
}

impl Rules {
    pub fn new() -> Self {
        Self { substitutions: HashMap::new() }
    }

    pub fn insert(&mut self, variable: Variable, substitution: Vec<Variable>) {
        self.substitutions.insert(variable, substitution);
    }

    pub fn get(&self, variable: &Variable) -> Vec<Variable> {
        match self.substitutions.get(variable) {
            Some(substitution) => {
                substitution.clone()
            }

            None => vec![variable.clone()],
        }
    }
}

fn apply(rules: &Rules, word: Word) -> Word {
    word.into_iter().fold(Vec::new(), |mut acc, variable| {
        let substitution = rules.get(&variable);
        for var in substitution {
            acc.push(var.clone());
        }
        acc
    })
}

fn render(word: &Word, renderer: &mut Renderer) {
    for variable in word {
        match variable.symbol {
            'F' => {
                renderer.forward();
            }

            '-' => {
                renderer.left();
            }

            '+' => {
                renderer.right();
            }

            '[' => {
                renderer.push();
            }

            ']' => {
                renderer.pop();
            }

            _ => {
                // do nothing
            }
        }
    }
}
