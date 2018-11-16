use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Variable {
    pub symbol: char,
}

impl Variable {
    pub fn new(symbol: char) -> Self {
        Self { symbol }
    }
}

pub type Word = Vec<Variable>;

pub struct Rules {
    substitutions: HashMap<Variable, Vec<Variable>>
}

impl Rules {
    pub fn new() -> Self {
        Self { substitutions: HashMap::new() }
    }

    pub fn insert(&mut self, variable: Variable, substitution: Vec<Variable>) {
        self.substitutions.insert(variable, substitution);
    }

    pub fn apply(&self, word: Word) -> Word {
        word.into_iter().fold(Vec::new(), |mut acc, variable| {
            let substitution = self.get(&variable);
            for var in substitution {
                acc.push(var.clone());
            }
            acc
        })
    }

    fn get(&self, variable: &Variable) -> Vec<Variable> {
        match self.substitutions.get(variable) {
            Some(substitution) => {
                substitution.clone()
            }

            None => vec![variable.clone()],
        }
    }

}


