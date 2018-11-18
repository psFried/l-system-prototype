# L-system Prototype
A prototype to test a few ideas.

## L-system
An [L-system][l-system] or Lindenmyer-system is a

> parallel rewriting system and a type of formal grammar. An L-system consists of an alphabet of symbols that can be used to make strings, a collection of production rules that expand each symbol into some larger string of symbols, an initial "axiom" string from which to begin construction, and a mechanism for translating the generated strings into geometric structures. L-systems were introduced and developed in 1968 by Aristid Lindenmayer, a Hungarian theoretical biologist and botanist at the University of Utrecht.

## Goal
Inspired by the [Fractal Tree][video] coding challenge, we want to recreate that image with Rust. Furthermore we want to explore ways of working with L-systems in such a way that is is easy to change a visualization methods.

## Progress
### [Initial Implementation](https://github.com/columbus-elst-connection/l-system-prototype/tree/3101fd714bff5284cec72f849bee3707d9570ca8)

![The first L-system rendered by the prototype](https://columbus-elst-connection.github.io/l-system-prototype/image/koch.png)

The initial implementation rendered the famous [Koch curve][koch] from the following L-system.

```plain
start: F
F -> F-F++F-F
```

This was implemented by hand with

```rust
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Variable {
    F,
    Plus,
    Minus,
}

type Word = Vec<Variable>;

type Rules = HashMap<Variable, Vec<Variable>>;

fn main() {
    let mut rules: Rules = HashMap::new();
    rules.insert(
        Variable::F,
        vec![
            Variable::F,
            Variable::Minus,
            Variable::F,
            Variable::Plus,
            Variable::Plus,
            Variable::F,
            Variable::Minus,
            Variable::F,
        ],
    );
    
    let mut word = vec![
        Variable::F,
    ];
}
```

The heart of the L-system is the `apply` functions which takes a words and replaces all the occurrences of a variable with their substitution, leaving the original variable when there is no substitution.

```rust
fn apply(rules: &Rules, word: Word) -> Word {
    word
        .into_iter()
        .fold(Vec::new(), |mut acc, variable|{
            match rules.get(&variable) {
                Some(substitution) => {
                    for var in substitution {
                        acc.push(var.clone());
                    }
                }

                None => {
                    acc.push(variable)
                }
            }
            acc
        })
}
```

The image is drawn on screen with the [`turtle` crate][turtle]. A `draw` function accepts a word to draw and a turtle that will do the drawing.

```rust

fn draw<C>(word: &Word, turtle: &mut Turtle, c: C)
where
    C: Into<Config>,
{
    let config: Config = c.into();
    for variable in word {
        match variable {
            Variable::F => {
                turtle.forward(config.step);
            }

            Variable::Minus => {
                turtle.left(config.angle);
            }

            Variable::Plus => {
                turtle.right(config.angle);
            }
        }
    }
}

struct Config {
    step: f64,
    angle: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            step: 100.0,
            angle: 60.0,
        }
    }
}
```

#### Pros & Cons
##### Pros
The implementation was straight forward with with each part of L-systems, i.e. definition, generation and visualization, having a clear counter part in code.

##### Cons
The visualization is tightly bound to the definition of the L-systems. I could not change the definition of the L-system without needing to change the visualization.

#### Considerations
The implementation does not does not support all features of turtle graphics. Most notable it the lack to push and pop states from the stack.

### [Renderer Trait](https://github.com/columbus-elst-connection/l-system-prototype/tree/c3966686ce0f35467d1905a5525237c851dae293)

I created an `Renderer` trait with a [`prototype::render::crab::Crab`](https://github.com/columbus-elst-connection/l-system-prototype/blob/c3966686ce0f35467d1905a5525237c851dae293/src/render/crab.rs) and a [`prototype::render::string::Collector`](https://github.com/columbus-elst-connection/l-system-prototype/blob/c3966686ce0f35467d1905a5525237c851dae293/src/render/string.rs) implementation.

```rust
pub trait Renderer {
    fn forward(&mut self);

    fn left(&mut self);

    fn right(&mut self);
}
```

The `Collector` shows the generated string

```plain
F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F-F-F++F-F-F-F++F-F++F-F++F-F-F-F++F-F
```

#### Pros & Cons
##### Pros
This allows for swapping the rendering implementation more easily.

##### Cons
This is very much geared to a ~turtle~crab-graphics rendition.

#### Considerations
Should we have have more general render contact? The [book][] extends crab-graphics to three dimensions. In what direction do we want to go?

### [Push & Pop](https://github.com/columbus-elst-connection/l-system-prototype/tree/bf6b91dd32f116ad31f39ebd42e7af69e506320a)

![Rendition of example 7 of the L-system wikipedia](https://columbus-elst-connection.github.io/l-system-prototype/image/plant.png)

The crab can now push it's state on a `[` and pop to that state on `]`. The `Renderer` trait is extended with `push` and `pop` methods.

##### Considerations
For a 3 dimensional crab we could provide an `Renderer3d` that inherits all the functions of `Renderer` and add some more.

### [Symbol based Variables](https://github.com/columbus-elst-connection/l-system-prototype/tree/59fded4a47315591e92115eec5373872e3792655)
In a push towards a dynamic representation of rules we swapped the enum-based implementation to a symbol based implementation.

```rust
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Variable {
    symbol: char,
}
```

#### Pros & Cons
##### Pros
It allows for a dynamic definition of L-system rules.

##### Cons
We have lost the compile time check.

#### Considerations
With the crab-graphics interpretation we already lost the compile time check because we have to ignore the symbols other then `F+-[]`, so there is little lost with this design.

### [WIP] Parsing
We are working on a struct based [parser combinator][combinator] framework. The core abstraction issuecomment

```rust
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
```

Each building block, or combinator implements the `Parser` trait. For example a `at_least` parses at least `n` of the things that `parser` parses.

```elm
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
```

#### Pros & Cons
##### Pros
Besides lifetime issues and the occasional `PhantomData`, the struct based parser is simple to work with. The process is largely "mechanize-able", i.e. for a type of parser like character, any, map

1. Create a struct containing the necessary fields,
2. Implement `Parser` for struct.

All of this is "standard" Rust.

##### Cons
At times it feels tedious to spell everything out.

#### Considerations
I feel that Parser API could be a bit more lean. It seems very heavy on the generics side.

[l-system]: https://en.wikipedia.org/wiki/L-system
[video]: https://www.youtube.com/watch?v=E1B4UoSQMFw
[koch]: https://en.wikipedia.org/wiki/Koch_snowflake
[turtle]: https://turtle.rs/
[book]: http://algorithmicbotany.org/papers/abop/abop.pdf
[combinator]:https://en.wikipedia.org/wiki/Parser_combinator 
