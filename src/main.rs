use parser::parse;
use api::{LSystem, SymbolIterator, Renderer, Symbol};
use renderer::Crab;
use std::fmt::Debug;
use std::hash::Hash;
use clap::{App, Arg};

fn main() {
    Crab::global_init();

    let parsed_args = App::new("l-sysem")
            .about("Renders an l-system")
            .arg(Arg::with_name("iterations")
                    .short("n")
                    .long("iterations")
                    .takes_value(true)
                    .default_value("1")
                    .help("The number of token replacement iterations to perform"))
            .arg(Arg::with_name("l-system-file")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .required(true)
                    .help("file containing the complete description of the l-system"))
            .arg(Arg::with_name("axiom")
                    .short("a")
                    .long("axiom")
                    .takes_value(true)
                    .required(true)
                    .help("The starting axiom to be used as input to the l-system"))
            .get_matches();

    let axiom_string = parsed_args.value_of("axiom").unwrap(); // safe unwrap since axiom is required
    let iterations = parsed_args.value_of("iterations").unwrap().parse::<usize>().unwrap_or_else(|_| {
        eprintln!("iterations argument must be a positive integer");
        ::std::process::exit(1);
    });
    let file = parsed_args.value_of("l-system-file").unwrap(); // safe unwrap since l-system-file is required

    let axiom: Vec<char> = axiom_string.chars().collect();

    let LSystem {mut rules, render_config} = parse(file)
        .expect("Failed to parse definition of L-system");

    let mut renderer = Crab::new(render_config);
    render(rules.symbol_iterator(iterations, axiom), &mut renderer);
}

fn render<T: Symbol>(iter: SymbolIterator<T>, renderer: &mut dyn Renderer) where T: Copy + Eq + Debug + Hash {
    for symbol in iter {
        let instruction = symbol.to_rendering_instruction();
        //eprintln!("Rendering symbol: {:?}, instruction: {:?}", symbol, instruction);
        renderer.render(instruction);
    }
    renderer.finish();
}
