use std::cell::RefCell;
use std::rc::Rc;

// Allows multiple owners and mutation of value
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn example() {
    let value = Rc::new(RefCell::new(1));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("ListWithRefCell a: {:?}", a);
    println!("ListWithRefCell b: {:?}", b);
    println!("ListWithRefCell c: {:?}", c);

    *value.borrow_mut() = 17;

    println!("ListWithRefCell a: {:?}", a);
    println!("ListWithRefCell b: {:?}", b);
    println!("ListWithRefCell c: {:?}", c);
}