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

    let mut rules: Rules = HashMap::new();
    rules.insert(
        Variable::F,
        vec![
            Variable::F,
            Variable::F,
        ],
    );
    rules.insert(
        Variable::X,
        vec![
            Variable::F,
            Variable::Plus,
            Variable::Open,
            Variable::Open,
            Variable::X,
            Variable::Close,
            Variable::Minus,
            Variable::X,
            Variable::Close,
            Variable::Minus,
            Variable::F,
            Variable::Open,
            Variable::Minus,
            Variable::F,
            Variable::X,
            Variable::Close,
            Variable::Plus,
            Variable::X
        ],
    );

    let mut word = vec![Variable::X];

    for _ in 0..n {
        word = apply(&rules, word);
    }


    let mut collector = Collector::new();
    let mut turtle = Turtle::new();
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
enum Variable {
    F,
    X,
    Plus,
    Minus,
    Open,
    Close,
}

type Word = Vec<Variable>;

type Rules = HashMap<Variable, Vec<Variable>>;

fn apply(rules: &Rules, word: Word) -> Word {
    word.into_iter().fold(Vec::new(), |mut acc, variable| {
        match rules.get(&variable) {
            Some(substitution) => {
                for var in substitution {
                    acc.push(var.clone());
                }
            }

            None => acc.push(variable),
        }
        acc
    })
}

fn render(word: &Word, renderer: &mut Renderer) {
    for variable in word {
        match variable {
            Variable::F => {
                renderer.forward();
            }

            Variable::Minus => {
                renderer.left();
            }

            Variable::Plus => {
                renderer.right();
            }

            Variable::Open => {
                renderer.push();
            }

            Variable::Close => {
                renderer.pop();
            }

            _ => {
                // do nothing
            }
        }
    }
}
