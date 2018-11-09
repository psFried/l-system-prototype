extern crate prototype;
extern crate turtle;

use prototype::render::string::Collector;
use prototype::render::Renderer;
use std::collections::HashMap;
use std::env;

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

    let mut word = vec![Variable::F];

    for _ in 0..n {
        word = apply(&rules, word);
    }


    let mut renderer = Collector::new();

    render(&word, &mut renderer);

    println!("{}", renderer);
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
        }
    }
}
