extern crate prototype;
extern crate turtle;

use prototype::parser::parse;
use prototype::system::{Variable, Word};
use prototype::render::crab::Crab;
use prototype::render::collection::batch;
use prototype::render::string::{Reporter, Collector};
use prototype::render::Renderer;
use std::env;
use turtle::Turtle;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args
        .get(1)
        .unwrap_or(&String::from("1"))
        .parse::<i32>()
        .expect("enter a valid number as first argument");

    let rules = parse("systems/plant.ls")
        .expect("a definition of a L-system");

    let mut word = vec![Variable::new('X')];

    word = rules.generation(word, n);

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
