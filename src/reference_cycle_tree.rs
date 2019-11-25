use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn new(value: i32, children: RefCell<Vec<Rc<Node>>>, parent: RefCell<Weak<Node>>) -> Self {
        Node{
            value,
            children,
            parent,
        }
    }
}

fn print_ref_count(label: &str, node: &Rc<Node>) {
    // pass a reference to a Rc reference to avoid affecting Rc strong_count and weak_count increase
    println!("{} strong: {} weak: {}", label, Rc::strong_count(&node), Rc::weak_count(&node));
}

pub fn example() {
    let leaf = Rc::new(Node::new(
        3,
        RefCell::new(vec![]),
        // We can create an empty Weak reference, which will .upgrade() to None
        RefCell::new(Weak::new()),
    ));
    print_ref_count("leaf created", &leaf);

    // create a scope to simulate branch being destroyed
    {
        let branch = Rc::new(Node::new(
            5,
            RefCell::new(vec![Rc::clone(&leaf)]),
            RefCell::new(Weak::new()),
        ));

        // println!("leaf after branch created {:?}", leaf);

        // We .downgrade() the branch to weak ref
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        print_ref_count("branch", &branch);
        print_ref_count("leaf", &leaf);
    }

    // We need to .borrow() the value inside the RefCell first
    println!("leaf.parent {:?}", leaf.parent.borrow().upgrade());
    print_ref_count("leaf", &leaf);
}