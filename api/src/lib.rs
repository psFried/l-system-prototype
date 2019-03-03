use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq)]
pub enum RendererInstruction<T: PartialEq + Eq + Hash + Debug + Copy + Clone> {
    Push,
    Pop,
    Render(T),
}


pub struct Rule<T: PartialEq + Eq + Hash + Debug + Copy + Clone> {
    pub match_input: T,
    pub productions: Vec<T>
}

impl<T: PartialEq + Eq + Hash + Debug + Copy + Clone> Rule<T> {
    pub fn new(match_input: T, productions: Vec<T>) -> Rule<T> {
        Rule { match_input, productions }
    }
}

pub struct LSystemRules<T: PartialEq + Eq + Hash + Debug + Copy + Clone> {
    rules: HashMap<T, Vec<T>>,
}

impl <T: PartialEq + Eq + Hash + Debug + Copy + Clone> LSystemRules<T> {

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

    pub fn symbol_iterator(&mut self, max_stack_depth: usize, start_symbol: T) -> SymbolIterator<T> {
        SymbolIterator {
            initial: Some(start_symbol),
            rules: self,
            max_stack_depth,
            stack: Vec::with_capacity(max_stack_depth),
        }
    }
}


pub struct SymbolIterator<'a, T: PartialEq + Eq + Hash + Debug + Copy + Clone> {
    initial: Option<T>,
    rules: &'a LSystemRules<T>,
    max_stack_depth: usize,
    stack: Vec<std::vec::IntoIter<T>>,
}

impl<T: PartialEq + Eq + Hash + Debug + Copy + Clone> SymbolIterator<'_, T> {

    fn pop_symbol(&mut self) -> Option<T> {
        let first_symbol = self.stack.last_mut().and_then(|symbols| {
            symbols.next()
        });
        if first_symbol.is_none() {
            //println!("popping stack");
            self.stack.pop();
        }
        first_symbol
    }

}

impl<T: PartialEq + Eq + Hash + Debug + Copy + Clone> Iterator for SymbolIterator<'_, T> {
    type Item = RendererInstruction<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // special case for the very beginning of the iterator
        if let Some(start) = self.initial.take() {
            self.stack.push(vec![start].into_iter());
            return Some(RendererInstruction::Push);
        }

        if self.stack.is_empty() {
            None
        } else {
            let sym = self.pop_symbol();
            if let Some(symbol) = sym {
                if self.stack.len() < self.max_stack_depth {
                    let next_list = self.rules.apply(symbol);
                    self.stack.push(next_list.into_iter());
                    // println!("Pushing onto stack from: {:?}, new stack len: {}", symbol, self.stack.len());
                    Some(RendererInstruction::Push)
                } else {
                    // println!("Rendering symbol: {:?}", symbol);
                    Some(RendererInstruction::Render(symbol))
                }
            } else {
                // println!("Popped from stack, new len: {}", self.stack.len());
                Some(RendererInstruction::Pop)
            }
        }
    }
}

pub trait Renderer<T> {
    fn push(&mut self);
    fn pop(&mut self);
    fn render(&mut self, instruction: T);

    fn flush(&mut self);
}

#[cfg(test)]
mod test {
    use super::*;
    use super::RendererInstruction::*;

    #[test]
    fn given_a_rule_system_where_every_symbol_is_referenced_the_rules_yeild_the_expected_symbols() {
        let mut system = LSystemRules::new()
                .add('a', vec!['b', 'c'])
                .add('b', vec!['c'])
                .add('c', vec!['a', 'b', 'c']);

        let result = system.symbol_iterator(3, 'c').take(36).collect::<Vec<_>>();

        let expected = vec![
            Push, Push, Push,
            Render('b'), Render('c'),
            Pop, Push,
            Render('c'),
            Pop, Push,
            Render('a'), Render('b'), Render('c'),
            Pop, Pop, Pop
        ];
        println!("result: {:?}", result);
        assert_eq!(result, expected);
    }
}
