use parser::parse;
use api::{LSystem, LSystemRules, SymbolIterator, Renderer, Symbol, RendererConfig};
use renderer::Crab;
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

    let LSystem {mut rules, render_config} = parse("systems/plant.ls")
        .expect("a definition of a L-system");

    let mut renderer = Crab::new(render_config);

    render(rules.symbol_iterator(n, vec!['X']), &mut renderer);
    println!("Finished");
}


fn render<T: Symbol>(iter: SymbolIterator<T>, renderer: &mut Renderer) where T: Copy + Eq + Debug + Hash {
    for symbol in iter {
        let instruction = symbol.to_rendering_instruction();
        println!("symbol: {:?}, instruction: {:?}", symbol, instruction);
        renderer.render(instruction);
    }
    renderer.flush();
}
