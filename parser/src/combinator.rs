use std::marker::PhantomData;

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError>;
}

impl <'a, T, F> Parser<'a, T> for F where F: Fn(&'a str) -> Result<(T, &'a str), ParseError> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError> {
        self(input)
    }
}

pub fn optional<'a, T: 'static>(parser: impl Parser<'a, T>) -> impl Parser<'a, Option<T>> {
    Optional {
        inner: parser,
        _phantom: PhantomData,
    }
}

struct Optional<'a, T, P: Parser<'a, T>> {
    inner: P,
    _phantom: PhantomData<&'a T>,
}

impl <'a, T, P> Parser<'a, Option<T>> for Optional<'a, T, P> where P: Parser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<(Option<T>, &'a str), ParseError> {
        match self.inner.parse(input) {
            Ok((value, rem)) => Ok((Some(value), rem)),
            Err(e) => Ok((None, input))
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum ParseError {
    IO,
    ExpectingCharacter(char),
    ExpectingEOF,

    ExpectingString(String),
    ExpectingPredicate,
    EndOfInput,
    ExpectingOneOfToParse,
    GenericError,
    Custom(String),
}


pub type ParseResult<'a, T> = Result<(T, &'a str), ParseError>;

pub struct Character {
    character_to_match : char,
}

impl Character {
    pub fn new<'a>(character_to_match: char) -> impl Parser<'a, char> {
        Self { character_to_match }
    }
}

pub fn character<'a>(character_to_match: char) -> impl Parser<'a, char> {
    Character::new(character_to_match)
}

impl<'a> Parser<'a, char> for Character {
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), ParseError> {
        if input.starts_with(self.character_to_match) {
            Ok((self.character_to_match, &input[1..]))
        } else {
            Err(ParseError::ExpectingCharacter(self.character_to_match))
        }
    }
}

pub fn many<'a, T: 'static>(parser: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> {
    at_least(0, parser)
}


pub struct AtLeast<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    n: u8,
    parser: P,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, P> AtLeast<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    pub fn new(n: u8, parser: P) -> Self {
        AtLeast { n, parser, phantom: PhantomData }
    }
}

pub fn at_least<'a, T: 'static>(n: u8, parser: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> {
    AtLeast::new(n, parser)
}

impl<'a, T, P> Parser<'a, Vec<T>> for AtLeast<'a, T, P> where P: Parser<'a, T> + Sized {
    fn parse(&self, input: &'a str) -> Result<(Vec<T>, &'a str), ParseError> {
        let mut result = vec![];
        let mut source = input;
        let mut count = self.n;
        while count > 0 {
            let attempt = self.parser.parse(source);
            match attempt {
                Ok((value, rest)) => {
                    result.push(value);
                    source = rest;
                }

                Err(e) => {
                    return Err(e);
                }
            }
            count -= 1;
        }
        loop {
            let attempt = self.parser.parse(source);
            match attempt {
                Ok((value, rest)) => {
                    result.push(value);
                    source = rest;
                }

                Err(_) => {
                    break;
                }
            }
        }
        Ok((result, source))
    }
}

pub struct Any<F> where F: Fn(char) -> bool + Sized {
    predicate: F,
}

impl<F> Any<F> where F: Fn(char) -> bool + Sized {
    pub fn new(predicate: F) -> Self {
        Any { predicate }
    }
}

pub fn any<'a, F>(predicate: F) -> impl Parser<'a, char> where F: Fn(char) -> bool + Sized {
    Any::new(predicate)
}

impl<'a, F> Parser<'a, char> for Any<F> where F: Fn(char) -> bool + Sized {
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), ParseError> {
        let character = input.chars().next();
        match character {
            Some(c) => {
                if (self.predicate)(c) {
                    Ok((c, &input[1..]))
                } else {
                    Err(ParseError::ExpectingPredicate)
                }
            },

            None => {
                Err(ParseError::EndOfInput)
            }
        }
    }
}

pub struct Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {

    parser: P,
    map: F,
    phantom: PhantomData<&'a I>,
}

impl<'a, I, O, P, F> Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    pub fn new(parser: P, map: F) -> Self {
        Map { parser, map, phantom: PhantomData }
    }
}

pub fn map<'a, I, O, P, F>(parser: P, map: F) -> impl Parser<'a, O> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    Map::new(parser, map)
}

impl<'a, I, O, P, F> Parser<'a, O> for Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    fn parse(&self, input: &'a str) -> Result<(O, &'a str), ParseError> {
        let attempt = self.parser.parse(input);
        attempt.map(|(v, rest)|{ ((self.map)(v), rest)})
    }
}

pub fn flat_map<'a, I, F, E, O>(a: impl Parser<'a, I>, mapper: F) -> impl Parser<'a, O> where F: Fn(I) -> Result<O, E>, E: Into<ParseError> {
    move |input: &'a str| {
        let (a, remaining) = a.parse(input)?;
        let b = mapper(a).map_err(Into::into)?;
        Ok((b, remaining))
    }
}

pub fn recognize<'a, T>(p: impl Parser<'a, T>) -> impl Parser<'a, &'a str> {
    move |input: &'a str| {
        let (_, rem) = p.parse(input)?;
        let match_len = input.len() - rem.len();
        let matched = &input[..match_len];
        Ok((matched, rem))
    }
}

// pub struct OneOf<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
//     options: Vec<P>,
//     phantom: PhantomData<&'a T>,
// }

// impl<'a, T, P> OneOf<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
//     pub fn new(options: Vec<P>) -> Self {
//         Self { options, phantom: PhantomData }
//     }
// }

pub fn one_of<'a, T>(options: Vec<Box<dyn Parser<'a, T>>>) -> impl Parser<'a, T> where T: 'a {
    move |input: &'a str| {
        for ref parser in options.iter() {
            let attempt = parser.parse(input);
            if attempt.is_ok() {
                return attempt;
            }
        }
        Err(ParseError::ExpectingOneOfToParse)
    }
}

// impl<'a, T, P> Parser<'a, T> for OneOf<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
//     fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError> {
//         for ref parser in &self.options {
//             let attempt = parser.parse(input);
//             if attempt.is_ok() {
//                 return attempt
//             }
//         }
//         Err(ParseError::ExpectingOneOfToParse)
//     }
// }

pub struct Literal<'p>(&'p str);
impl <'a, 'p> Parser<'a, &'a str> for Literal<'p> {
    fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), ParseError> {
        if input.starts_with(self.0) {
            let len = self.0.len();
            let substr = &input[..len];
            let rem = &input[len..];
            Ok((substr, rem))
        } else {
            Err(ParseError::ExpectingString(self.0.to_owned()))
        }
    }
}

pub fn literal<'p>(match_exactly: &'p str) -> Literal<'p> {
    Literal(match_exactly)
}

pub fn skip_spaces(input: &str) -> Result<((), &str), ParseError> {
    let byte_count = input.chars().take_while(|c| *c == ' ' || *c == '\t').count();
    Ok(((), &input[byte_count..]))
}

fn eof(input: &str) -> Result<((), &str), ParseError> {
    if input.is_empty() {
        Ok(((), input))
    } else {
        Err(ParseError::ExpectingEOF)
    }
}

pub fn complete<'a, T>(parser: impl Parser<'a, T>) -> impl Parser<'a, T> {
    move |input| {
        let (res, rem) = parser.parse(input)?;
        let (_, rem) = eof(rem)?;
        Ok((res, rem))
    }
}

#[macro_export]
macro_rules! parse_sequence {
    ($(let $name:ident = $parser:expr),+ => $finish:expr ) => {{
        |input| {
            let rem = input;
            $(
                let ($name, rem) =$parser.parse(rem)?;
            )*
            let result = $finish;
            Ok((result, rem))
        }
    }};
}

#[macro_export]
macro_rules! parse_sequence_ignore_spaces {
    ($(let $name:ident = $parser:expr),+ => $finish:expr ) => {{
        |input| {
            let rem = input;
            $(
                let (_, rem) = $crate::combinator::skip_spaces(rem)?;
                let ($name, rem) = $crate::combinator::Parser::parse(&$parser, rem)?;
            )*
            let (_, rem) = $crate::combinator::skip_spaces(rem)?;
            let result = $finish;
            Ok((result, rem))
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_literal_string() {
        let (res, rem) = literal("foo").parse("foo").unwrap();
        assert_eq!("foo", res);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_sequence_ignore_spaces_using_macro() {

        let parser = parse_sequence_ignore_spaces!{
            let a = character('A'),
            let _foo = literal("foo"),
            let c = character('C')
            =>
            (a, c)
        };
        let (result, rem) = parser.parse(" \t A foo\t C  \t ").unwrap();
        assert_eq!(('A', 'C'), result);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_sequence_using_macro() {
        let parser = parse_sequence!{
            let a = character('A'),
            let b = character('b')
            =>
            (a, b)
        };
        let (result, rem) = parser.parse("Ab").expect("failed to parse");
        assert_eq!(('A', 'b'), result);
        assert!(rem.is_empty());
    }



    #[test]
    fn parse_a_character() {
        let input = "ABCD";
        let parser = character('A');

        let actual = parser.parse(input);

        let expected = Ok(('A', "BCD"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_many_a_characters() {
        let input = "AAABCD";
        let parser = many(character('A'));

        let actual = parser.parse(input);

        let expected = Ok((vec!['A', 'A', 'A'], "BCD"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_at_least_2_a_characters() {
        let input = "AAABCD";
        let parser = at_least(2, character('A'));

        let actual = parser.parse(input);

        let expected = Ok((vec!['A', 'A', 'A'], "BCD"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_any_character() {
        let input = "AAABCD";
        let parser = any(|c: char|{ c.is_alphabetic() });

        let actual = parser.parse(input);

        let expected = Ok(('A', "AABCD"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn map_many_a_character_to_length() {
        let input = "AAABCD";
        let parser = map(many(character('A')), |cs|{cs.len()});

        let actual = parser.parse(input);

        let expected = Ok((3, "BCD"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn  parse_one_of_b_or_a_characters() {
        let input = "AAABCD";
        let parser = one_of(vec![
            character('B'),
            character('A'),
        ]);

        let actual = parser.parse(input);

        let expected = Ok(('A', "AABCD"));
        assert_eq!(actual, expected);
    }
}
