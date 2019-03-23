mod combinator;

use std::path::Path;
use std::fs::File;
use std::io::{Error, Read};
use std::fmt::Debug;
use std::hash::Hash;
use api::{Rule, LSystemRules, LSystem, RendererConfig};
#[macro_use] use self::combinator::{Parser, ParseError, many, map, at_least, character, literal, one_of};

pub fn parse<P>(path: P) -> Result<LSystem<char>, ParseError>
where P: AsRef<Path> {
    let mut file = File::open(path).map_err(to_parse_error)?;
    let mut input = String::new();
    file.read_to_string(&mut input).map_err(to_parse_error)?;
    let (remaining_input, render_config) = parse_config(&input)?;
    let rules = parse_rules(remaining_input)?;
    Ok(LSystem {
        rules: LSystemRules::from_rules(rules),
        render_config,
    })
}

fn to_parse_error(_io_error: Error) -> ParseError {
    ParseError::IO
}

fn parse_config(input: &str) -> Result<(&str, RendererConfig), ParseError> {
    // TODO: actually parse the config
    let config = RendererConfig {
        step: 2.0,
        angle: 45.0,
        step_multiplier: 1.5,
    };
    Ok((input, config))
}

fn non_ws_char(input: &str) -> Result<(char, &str), ParseError> {
    let c = input.chars().next().ok_or(ParseError::EndOfInput)?;
    if c.is_ascii_graphic() {
        Ok((c, &input[1..]))
    } else {
        Err(ParseError::ExpectingPredicate)
    }
}

fn parse_symbol<'a>() -> impl Parser<'a, char> {
    parse_sequence_ignore_spaces!{
        let c = non_ws_char
        =>
        c
    }
}

fn skip_all_ws(input: &str) -> Result<((), &str), ParseError> {
    let byte_count = input.chars().take_while(|c| c.is_whitespace()).map(|c| c.len_utf8()).sum();
    Ok(((), &input[byte_count..]))
}

fn newline<'a>() -> impl Parser<'a, ()> {
    map(one_of(vec![literal("\n"), literal("\r\n"), literal("\r")]), |_| () )
}


fn parse_rules(input: &str) -> Result<Vec<Rule<char>>, ParseError> {
    let parse_rule = parse_sequence_ignore_spaces!{
        let _extra_ws = skip_all_ws,
        let to_match = parse_symbol(),
        let _separator = literal("=>"),
        let replacements = at_least(1, parse_symbol()),
        let _newline = newline()
        =>
        Rule::new(to_match, replacements)
    };
    let parser = at_least(1, parse_rule);

    parser.parse(input).map(|success| {
        success.0
    })
}


// fn config_parser<'a>() -> impl Parser<'a, RendererConfig> {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_renderer_config_is_parsed() {
        let input = r##"
        render:
        step = 40.5

        "##;
    }

    #[test]
    fn valid_rules_are_parsed() {
        let input = r##"
        A => BC [ [ +D D

        B=>AAA


        C   =>    DAD
        D=> ABC

        "##;
        let expected = vec![
            Rule::new('A', vec!['B', 'C', '[', '[', '+', 'D', 'D']),
            Rule::new('B', vec!['A', 'A', 'A']),
            Rule::new('C', vec!['D', 'A', 'D']),
            Rule::new('D', vec!['A', 'B', 'C'])
        ];
        let actual = parse_rules(input).unwrap();
        assert_eq!(expected, actual);
    }

}
