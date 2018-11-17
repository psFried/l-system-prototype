use std::path::Path;
use std::fs::File;
use std::io::{Error, Read};

use super::system::{Rules, Variable};


pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<P>(&self, path: P) -> Result<Rules, ParseError>
    where P: AsRef<Path> {
        let mut file = File::open(path).map_err(to_parse_error)?;
        let mut input = String::new();
        file.read_to_string(&mut input).map_err(to_parse_error)?;
        let rules = parse_rules(&input);
        rules.map(to_rules)
    }
}

fn to_parse_error(io_error: Error) -> ParseError {
    ParseError::IO(io_error)
}

fn to_rules(tuple: (Rules, &str))-> Rules {
    tuple.0
}

#[derive(Debug)]
pub enum ParseError {
    IO(Error),
    GenericError,
}

fn parse_rules(input: &str) -> Result<(Rules, &str), ParseError> {
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
    Ok((rules, &input[..]))
}

