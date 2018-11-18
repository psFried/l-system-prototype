use std::marker::PhantomData;

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError>;
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    IO,
    ExpectingCharacter(char),
    ExpectingPredicate,
    EndOfInput,
    GenericError,
}

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

pub fn many<'a, T>(parser: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> {
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

pub fn at_least<'a, T>(n: u8, parser: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
