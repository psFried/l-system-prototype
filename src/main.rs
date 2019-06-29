use parser::parse;
use api::{LSystem, SymbolIterator, Renderer, Symbol};
use renderer::Crab;
use std::fmt::Debug;
use std::hash::Hash;

use std::env;

fn main() {
    Crab::global_init();
    let args: Vec<String> = env::args().collect(); // vec!["foo".to_string(), "3".to_string(), "XFX".to_string()]; //
    eprintln!("ARGS: {:?}", args);
    let n = args
        .get(1)
        .unwrap_or(&String::from("1"))
        .parse::<usize>()
        .expect("enter a valid number as first argument");

    let axiom_string = args.get(2).expect("Missing required second argument");
    let axiom: Vec<char> = axiom_string.chars().collect();

    let LSystem {mut rules, render_config} = parse("systems/plant.ls")
        .expect("Failed to parse definition of L-system");


    let mut renderer = Crab::new(render_config);
    render(rules.symbol_iterator(n, axiom), &mut renderer);
}


fn render<T: Symbol>(iter: SymbolIterator<T>, renderer: &mut Renderer) where T: Copy + Eq + Debug + Hash {
    for symbol in iter {
        let instruction = symbol.to_rendering_instruction();
        //eprintln!("Rendering symbol: {:?}, instruction: {:?}", symbol, instruction);
        renderer.render(instruction);
    }
    renderer.finish();
}
