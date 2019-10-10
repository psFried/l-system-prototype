mod combinator;

use std::fmt::Debug;
use std::str::FromStr;
use std::path::Path;
use std::fs::File;
use std::io::{Error, Read};
use api::{Rule, LSystemRules, LSystem, RendererConfig};
use self::combinator::{Parser, ParseResult, ParseError, map, flat_map, at_least, many, literal, one_of, any, recognize, optional};

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

fn to_parse_error(_io_error: Error) -> ParseError {
    ParseError::IO
}

fn decimal<'a>() -> impl Parser<'a, f64> {
    let decimal_str = recognize(parse_sequence!{
        let _before = any(|c| c.is_ascii_digit()),
        let _dot = literal("."),
        let _fractional = any(|c| c.is_ascii_digit())
        => () });
    flat_map(decimal_str, |dec| { dec.parse::<f64>().map_err(|_| ParseError::Custom("invalid decimal".to_owned()))})
}

fn config_value<'a, T: FromStr>() -> impl Parser<'a, T> {
    let recognize_value = recognize(at_least(1, any(|c| c != '\r' && c != '\n')));
    flat_map(recognize_value, |value| {
        let trimmed = value.trim();
        trimmed.parse::<T>().map_err(|e| {
            let err_msg = format!("Invalid config value: '{}
           '", trimmed);
           ParseError::Custom(err_msg)
        })
    })
}

/// We parse each `key = value` line into this representation so that we can support different value types with one parser
#[derive(Debug, Clone, PartialEq)]
enum ConfigItem {
    StartingStep(f64),
    StepMultiplier(f64),
    StartingAngle(f64),
    AngleMultiplier(f64),
    StartingLineWidth(f64),
    LineWidthMultiplier(f64),
    BackgroundColor(String),
    PenColor(String),
}

fn config_item<'a, T>(name: &'static str, fun: fn(T) -> ConfigItem) -> impl Parser<'a, ConfigItem>  where T: FromStr {
    parse_sequence_ignore_spaces!{
        let _ident = literal(name),
        let _eq = literal("="),
        let value = config_value::<'a, T>(),
        let _nl = newline()
        =>
        fun(value)
    }
}

fn float_config_item(value: &str, make: fn(f64) -> ConfigItem) -> Result<ConfigItem, ParseError> {
    value.parse().map_err(|e| {
        ParseError::Custom(format!("Invalid float value: '{}'", value))
    }).map(|float_val| {
        make(float_val)
    })
}

fn string_config_item(value: &str, make: fn(String) -> ConfigItem) -> Result<ConfigItem, ParseError> {
    if value.trim().is_empty() {
        return Err(ParseError::Custom("Missing config item value".to_owned()));
    }
    Ok(make(value.to_owned()))
}

fn parse_config(input: &str) -> Result<(RendererConfig, &str), ParseError> {
    let items_parser = many(one_of(vec![
        Box::new(config_item("starting_step", |v| ConfigItem::StartingStep(v))),
        Box::new(config_item("step_multiplier", |v| ConfigItem::StepMultiplier(v))),
        Box::new(config_item("starting_angle", |v| ConfigItem::StartingAngle(v))),
        Box::new(config_item("angle_multiplier", |v| ConfigItem::AngleMultiplier(v))),
        Box::new(config_item("starting_line_width", |v| ConfigItem::StartingLineWidth(v))),
        Box::new(config_item("line_width_multiplier", |v| ConfigItem::LineWidthMultiplier(v))),
        Box::new(config_item::<'_, String>("background_color", |v| ConfigItem::BackgroundColor(v))),
        Box::new(config_item::<'_, String>("pen_color", |v| ConfigItem::PenColor(v))),
    ]));
    let (results, rem) = items_parser.parse(input)?;
    let mut config = RendererConfig::default();
    for item in results {
        match item {
            ConfigItem::StartingStep(value) => config.starting_step = value,
            ConfigItem::StepMultiplier(value) => config.step_multiplier = value,
            ConfigItem::StartingAngle(value) => config.starting_angle = value,
            ConfigItem::AngleMultiplier(value) => config.angle_multiplier = value,
            ConfigItem::StartingLineWidth(value) => config.starting_line_width = value,
            ConfigItem::LineWidthMultiplier(value) => config.line_width_multiplier = value,
            ConfigItem::BackgroundColor(value) => config.background_color = value,
            ConfigItem::PenColor(value) => config.pen_color = value,
        }
    }
    Ok((config, rem))
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
    map(one_of(vec![
        Box::new(literal("\n")),
        Box::new(literal("\r\n")),
        Box::new(literal("\r"))
    ]), |_| () )
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
