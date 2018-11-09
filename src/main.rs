extern crate turtle;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _i in 0..360 {
        turtle.forward(3.0);

        turtle.right(1.0);
    }
}
