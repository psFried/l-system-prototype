use std::io::Error;

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

pub fn character(character_to_match: char) -> Character {
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
}
