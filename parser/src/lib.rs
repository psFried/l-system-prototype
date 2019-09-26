mod combinator;

use std::path::Path;
use std::fs::File;
use std::io::{Error, Read};
use api::{Rule, LSystemRules, LSystem, RendererConfig};
use self::combinator::{Parser, ParseResult, ParseError, map, at_least, literal, one_of, any};

pub fn parse<P>(path: P) -> Result<LSystem<char>, ParseError>
where P: AsRef<Path> {
    let mut file = File::open(path).map_err(to_parse_error)?;
    let mut input = String::new();
    file.read_to_string(&mut input).map_err(to_parse_error)?;
    let (system, _) = l_system_parser().parse(&input)?;

    Ok(system)
}

fn l_system_parser<'a>() -> impl Parser<'a, LSystem<char>> {
    parse_sequence_ignore_spaces!{
        let _config_header = literal("render"),
        let _colon = literal(":"),
        let _nl = newline(),
        let render_config = parse_config,
        let _rules_header = literal("rules"),
        let _colon = literal(":"),
        let _nl = newline(),
        let rules = rules_parser()
        =>
        LSystem { rules, render_config }
    }
}

// fn section<'a>(name: &'static str) -> impl Parser<'a, ()> {
//     parse_sequence_ignore_spaces!{
//         let _section_name = literal(name),
//         let _colon = literal(":"),
//         let _newline = newline()
//         => ()
//     }
// }

fn to_parse_error(_io_error: Error) -> ParseError {
    ParseError::IO
}

fn decimal<'a>() -> impl Parser<'a, f64> {
    let decimal_str = parse_sequence!{any(char::is_ascii_digit), literal("."), any(char::is_ascii_digit)};
    map(decimal_str, f64::)
}

macro_rules! config_item {
    ($name:expr, $val_type:ty) => {{
        parse_sequence_ignore_spaces!{
            let _ident = literal($name),
            let _eq = literal("="),
            let _nl = newline(),
            let value =
            =>
        }
    }};
}

fn parse_config(input: &str) -> Result<(RendererConfig, &str), ParseError> {
    // TODO: actually parse the config
    let config = RendererConfig {
        step: 2.0,
        angle: 45.0,
        step_multiplier: 1.5,
    };
    Ok((config, input))
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

fn rule_parser<'a>() -> impl Parser<'a, Rule<char>> {
    parse_sequence_ignore_spaces!{
        let _extra_ws = skip_all_ws,
        let to_match = parse_symbol(),
        let _separator = literal("=>"),
        let replacements = at_least(1, parse_symbol()),
        let _newline = newline()
        =>
        Rule::new(to_match, replacements)
    }
}

fn rules_parser<'a>() -> impl Parser<'a, LSystemRules<char>> {
    map(at_least(1, rule_parser()), LSystemRules::from_rules)
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

        rules:
        A => BCD
        B=>AAA
        C   =>    DAD
        D=> ABC
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
