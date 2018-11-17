use std::path::Path;

use super::system::{Rules, Variable};


pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<P>(&self, _path: P) -> Rules
    where P: AsRef<Path> {
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
        rules
    }
}
