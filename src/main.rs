extern crate turtle;

use turtle::Turtle;

fn main() {
    let word = vec![
        Variable::F,
        Variable::Minus,
        Variable::F,
        Variable::Plus,
        Variable::Plus,
        Variable::F,
        Variable::Minus,
        Variable::F
    ];

    let mut turtle = Turtle::new();
    turtle.set_heading(0.0);

    draw(&word, &mut turtle);
}

enum Variable {
    F,
    Plus,
    Minus,
}

type Word = Vec<Variable>;


fn draw(word: &Word, turtle: &mut Turtle) {
    for variable in word {
        match variable {
            Variable::F => { turtle.forward(100.0); }

            Variable::Minus => { turtle.left(60.0); }

            Variable::Plus => { turtle.right(60.0); }
        }
    }
}
