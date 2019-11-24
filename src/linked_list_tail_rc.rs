use std::rc::Rc;

// Reference Counted Rc<T>:
// counts the references to figure out
// to figure out whether the reference is still in use.
// Allocates on heap
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::*;

pub fn example() {
    // let a = Cons(5,
    //     Box::new(Cons(10,
    //         Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);

    let d = Rc::new(Cons(5,
        Rc::new(Cons(10, Rc::new(Nil)))
    ));
    println!("created d, refcount is {}", Rc::strong_count(&d));

    let _e = Cons(3, Rc::clone(&d));
    println!("cloned d inside e, refcount is {}", Rc::strong_count(&d));
    {
        let _f = Cons(4, Rc::clone(&d));
        println!("f is in scope, refcount is {}", Rc::strong_count(&d));
    }

    println!("f goes out of scope, refcount is {}", Rc::strong_count(&d));
}