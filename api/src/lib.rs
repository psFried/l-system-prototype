use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct RendererConfig {
    pub starting_step: f64,
    pub step_multiplier: f64,
    pub starting_angle: f64,
    pub angle_multiplier: f64,
    pub starting_line_width: f64,
    pub line_width_multiplier: f64,
    pub background_color: String,
    pub pen_color: String,
}

impl Default for RendererConfig {
    fn default() -> RendererConfig {
        RendererConfig {
            starting_step: 10.0,
            step_multiplier: 1.5,
            starting_angle: 45.0,
            angle_multiplier: 2.0,
            starting_line_width: 4.0,
            line_width_multiplier: 1.5,
            background_color: "white".to_owned(),
            pen_color: "black".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RendererInstruction {
    Forward,

    RotateLeft,
    RotateRight,

    Push,
    Pop,

    IncreaseStep,
    DecreaseStep,
    // IncreaseAngleIncrement,
    // DecreaseAngleIncrement,
    // IncreaseLineWidth,
    // DecreaseLineWidth,

    NoOp,
}

pub trait Symbol: Debug + Eq + Hash + Copy {
    fn to_rendering_instruction(&self) -> RendererInstruction;
}

impl Symbol for char {
    fn to_rendering_instruction(&self) -> RendererInstruction {
        match *self {
            'F' => RendererInstruction::Forward,
            '+' => RendererInstruction::RotateRight,
            '-' => RendererInstruction::RotateLeft,

            '[' => RendererInstruction::Push,
            ']' => RendererInstruction::Pop,

            '#' => RendererInstruction::IncreaseStep,
            '!' => RendererInstruction::DecreaseStep,

            _ => RendererInstruction::NoOp,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rule<T: Symbol> {
    pub match_input: T,
    pub productions: Vec<T>
}

impl<T: Symbol> Rule<T> {
    pub fn new(match_input: T, productions: Vec<T>) -> Rule<T> {
        Rule { match_input, productions }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LSystemRules<T: Symbol> {
    rules: HashMap<T, Vec<T>>,
}

impl <T: Symbol> LSystemRules<T> {

    pub fn new() -> LSystemRules<T> {
        LSystemRules {
            rules: HashMap::with_capacity(4),
        }
    }

    pub fn from_rules(rules: Vec<Rule<T>>) -> LSystemRules<T> {
        let mut system = LSystemRules::new();
        for rule in rules {
            system = system.add_rule(rule);
        }
        system
    }

    pub fn add_rule(self, rule: Rule<T>) -> Self {
        let Rule {match_input, productions} = rule;
        self.add(match_input, productions)
    }

    pub fn add(mut self, match_input: T, productions: Vec<T>) -> Self {
        self.rules.insert(match_input, productions);
        self
    }

    pub fn apply(&self, symbol: T) -> Vec<T> {
        self.rules.get(&symbol).cloned().unwrap_or_else(|| {
            vec![symbol]
        })
    }

    pub fn symbol_iterator(&mut self, iterations: usize, axiom: Vec<T>) -> SymbolIterator<T> {
        SymbolIterator {
            axiom: axiom.into_iter(),
            rules: self,
            iterations,
            stack: Vec::with_capacity(iterations),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LSystem<T: Symbol> {
    pub rules: LSystemRules<T>,
    pub render_config: RendererConfig,
}


pub struct SymbolIterator<'a, T: Symbol> {
    axiom: std::vec::IntoIter<T>,
    rules: &'a LSystemRules<T>,
    iterations: usize,
    stack: Vec<std::vec::IntoIter<T>>,
}

impl<T: Symbol> SymbolIterator<'_, T> {

    fn pop_next_symbol(&mut self) -> Option<T> {
        self.stack.last_mut().and_then(|s| s.next())
            .or_else(|| self.axiom.next())
    }

    fn clear_empty_stack_frames(&mut self) {
        while self.stack.last().filter(|s| s.len() == 0).is_some() {
            self.stack.pop();
        }
    }
}

impl<T: Symbol> Iterator for SymbolIterator<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.clear_empty_stack_frames();

        for _ in self.stack.len()..self.iterations {
            if let Some(symbol) = self.pop_next_symbol() {
                let replacement = self.rules.apply(symbol);
                self.stack.push(replacement.into_iter());
            } else {
                break;
            }
        }
        self.pop_next_symbol()
    }
}


pub trait Renderer {
    fn global_init() where Self: Sized {}

    fn new(renderer_config: RendererConfig) -> Self;

    fn render(&mut self, instruction: RendererInstruction) {
        match instruction {
            RendererInstruction::Push => self.push(),
            RendererInstruction::Pop => self.pop(),
            RendererInstruction::Forward => self.forward(),
            RendererInstruction::RotateLeft => self.rotate_left(),
            RendererInstruction::RotateRight => self.rotate_right(),
            RendererInstruction::IncreaseStep => self.increase_step(),
            RendererInstruction::DecreaseStep =>  self.decrease_step(),
            RendererInstruction::NoOp => { /* no-op */ }
        }
    }

    fn push(&mut self) {}
    fn pop(&mut self) {}

    fn forward(&mut self) {}
    fn rotate_left(&mut self) {}
    fn rotate_right(&mut self) {}

    fn increase_step(&mut self) {}
    fn decrease_step(&mut self) {}

    fn finish(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_a_rule_system_where_every_symbol_is_referenced_the_rules_yeild_the_expected_symbols() {
        let mut system = LSystemRules::new()
                .add('a', vec!['b', 'c'])
                .add('b', vec!['c'])
                .add('c', vec!['a', 'b', 'c']);

        let result1 = system.symbol_iterator(1, vec!['c']).collect::<Vec<_>>();
        assert_eq!(result1, vec!['a', 'b', 'c']);

        let result2 = system.symbol_iterator(1, vec!['a', 'b', 'c']).take(20).collect::<Vec<_>>();
        assert_eq!(result2, vec!['b', 'c', 'c', 'a', 'b', 'c']);

        let result3 = system.symbol_iterator(2, vec!['c']).collect::<Vec<_>>();
        assert_eq!(result2, result3);
    }

}
