use parser::parse;
use api::{LSystemRules, SymbolIterator, Renderer};
use renderer::{Crab, Config};
use std::fmt::Debug;
use std::hash::Hash;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args
        .get(1)
        .unwrap_or(&String::from("1"))
        .parse::<usize>()
        .expect("enter a valid number as first argument");

    let mut rules = parse("systems/plant.ls")
        .expect("a definition of a L-system");


    //let config = (200.0 / 2.5f64.powi(n as i32), 25.0);
    let config = Config {
        step: 300.0,
        step_multiplier: 1.8,
        angle: 45.0,
    };
    let mut renderer = Crab::new(config);

    render(rules.symbol_iterator(n, 'X'), &mut renderer);
    println!("Finished");
}


fn render<T>(iter: SymbolIterator<T>, renderer: &mut Renderer<T>) where T: Copy + Eq + Debug + Hash {
    use api::RendererInstruction;
    for instruction in iter {
        println!("instruction: {:?}", instruction);
        match instruction {
            RendererInstruction::Push => renderer.push(),
            RendererInstruction::Pop => renderer.pop(),
            RendererInstruction::Render(t) => renderer.render(t),
        }
    }
    renderer.flush();
}
