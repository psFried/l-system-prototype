use std::marker::PhantomData;

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError>;
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    IO,
    ExpectingCharacter(char),
    GenericError,
}

pub struct Character {
    character_to_match : char,
}

impl Character {
    pub fn new(character_to_match: char) -> Self {
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

pub struct Many<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    parser: P,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, P> Many<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    pub fn new(parser: P) -> Self {
        Self { parser, phantom: PhantomData }
    }
}

pub fn many<'a, T>(parser: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> {
    Many::new(parser)
}

impl<'a, T, P> Parser<'a, Vec<T>> for Many<'a, T, P> where P: Parser<'a, T> + Sized {
    fn parse(&self, input: &'a str) -> Result<(Vec<T>, &'a str), ParseError> {
        let mut result = vec![];
        let mut source = input;
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
}
