use std::ops::Deref;
use std::mem::drop;

mod linked_list_tail_box;
mod linked_list_tail_rc;
mod list_with_mutable_value;
mod reference_cycle;
mod reference_cycle_tree;


// Custom smart pointer.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> Self {
        MyBox(v)
    }
}

impl<T> Deref for MyBox<T> {
    // Type associated with Deref trait.
    type Target = T;
    
    fn deref(&self) -> &T {
        // Tuple contains only one element, which is returned
        &self.0
    }
}

// Drop trait implementation for CustomPointer
#[derive(Debug)]
struct CustomPointer<T> {
    value: T,
}

impl<T> CustomPointer<T> {
    fn new(value: T) -> Self {
        CustomPointer {
            value,
        }
    }
}

impl<T> Drop for CustomPointer<T> {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Allows multiple owners


fn main() {    
    // Dereference operator example:
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let w = MyBox::new(x);
    
    // Dereference coercion allows using smart pointers,
    // just as the normal pointers(&) with (*) dereference operator.
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);

    // Deref coercion example:
    // https://doc.rust-lang.org/book/ch15-02-deref.html
    hello("Rust");

    let a = Box::new("String Slice");
    let b = Box::new(String::from("String"));
    let c = MyBox::new(String::from("Rust"));
    let d = MyBox::new(String::from("Rust"));

    hello(&a);
    hello(&b);
    hello(&c);
    // Manual deref implementation.
    hello(&(*d)[..]);

    // Drop trait
    // https://doc.rust-lang.org/book/ch15-03-drop.html
    let v = CustomPointer::new("Hello drop!");

    println!("{:?}", v);

    println!("CustomSmartPointer created.");
    // Actually removes the value from memory.
    drop(v);
    println!("CustomSmartPointer dropped before the end of main.");
    // Value is no longer available.
    // println!("{}", v);

    linked_list_tail_box::example();
    linked_list_tail_rc::example();
    list_with_mutable_value::example();
    reference_cycle::example();
    reference_cycle_tree::example();
}
