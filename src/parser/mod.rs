mod combinator;

use std::path::Path;
use std::fs::File;
use std::io::{Error, Read};

use super::system::{Rules, Variable};
use self::combinator::{Parser, ParseError, many, character};

pub fn parse<P>(path: P) -> Result<Rules, ParseError>
where P: AsRef<Path> {
    let mut file = File::open(path).map_err(to_parse_error)?;
    let mut input = String::new();
    file.read_to_string(&mut input).map_err(to_parse_error)?;
    let rules = parse_rules(&input);
    rules.map(to_rules)
}

fn to_parse_error(_io_error: Error) -> ParseError {
    ParseError::IO
}

fn to_rules(tuple: (Rules, &str))-> Rules {
    tuple.0
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

fn parse_rule(input: &str) -> Result<((Variable, Vec<Variable>), &str), ParseError> {
    let result = (Variable::new('F'), vec!(
        Variable::new('F'),
        Variable::new('-'),
        Variable::new('F'),
        Variable::new('+'),
        Variable::new('+'),
        Variable::new('F'),
        Variable::new('-'),
        Variable::new('F'),
    ));
    Ok((result, &input[..]))
}

fn spaces<'a>() -> impl Parser<'a, Vec<char>> {
    many(character(' '))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_multiple_spaces() {
        let input = "    ";
        let parser = spaces();

        let actual = parser.parse(input);

        assert!(actual.is_ok());
    }
}
