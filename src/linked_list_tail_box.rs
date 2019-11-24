// Recursive data-structure.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

pub fn example() {
    // Recursive data structure with smart pointer:
    let list = Cons(
        1,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))),
    );
    println!("{:?}", list);
}