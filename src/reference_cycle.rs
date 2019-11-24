use std::cell::RefCell;
use std::rc::Rc;

// Allows multiple owners and mutations
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None
        }
    }
}

pub fn example() {
    println!("Reference cycle");
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a: {:?}", a);
    println!("a initial rc: {:?}", Rc::strong_count(&a));
    println!("a.tail(): {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("b: {:?}", b);
    println!("a rc after b is created: {:?}", Rc::strong_count(&a));
    println!("b initial rc: {:?}", Rc::strong_count(&b));
    println!("b.tail(): {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after mutating a: {}", Rc::strong_count(&a));
    println!("a rc count affter mutating a: {}", Rc::strong_count(&b));

    
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}