mod combinator;

use std::path::Path;
use std::fs::File;
use std::io::{Error, Read};
use std::fmt::Debug;
use std::hash::Hash;
use api::{Rule, LSystemRules};
use self::combinator::{Parser, ParseError, many, character};

pub fn parse<P>(path: P) -> Result<LSystemRules<char>, ParseError>
where P: AsRef<Path> {
    let mut file = File::open(path).map_err(to_parse_error)?;
    let mut input = String::new();
    file.read_to_string(&mut input).map_err(to_parse_error)?;
    let rules = parse_rules(&input)?;
    Ok(LSystemRules::from_rules(rules))
}

fn to_parse_error(_io_error: Error) -> ParseError {
    ParseError::IO
}


fn parse_rules(input: &str) -> Result<Vec<Rule<char>>, ParseError> {
    // TODO: actually parse input instead of returning dummy values
    let rules = vec![
        Rule::new('X', vec!['R', 'F', 'Y', 'F', 'R', 'F', 'Z', 'F']),
        Rule::new('Y', vec!['F', 'R', 'X', 'R', 'F']),
        Rule::new('Z', vec!['F', 'R', 'F', 'Y', 'F']),
    ];
    Ok(rules)
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
