// Transaction Log
use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
pub struct Node {
    pub value: String,
    pub next: SingleLink,
}

pub struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    length: u64,
}


impl Node {
    // A nice and short way of creating a new node
    pub fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }
}

// Append entries at the end and remove entries from the front
