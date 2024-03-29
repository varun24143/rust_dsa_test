// Transaction Log
use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
pub struct Node {
    pub value: String,
    pub next: SingleLink,
    pub prev: SingleLink, // This is to make the list to be double linked list, by allowing it to traverse back
}

pub struct BetterTransactionLog { // Revised as per doubly linked list
    head: SingleLink,
    tail: SingleLink,
    length: u64,
}

pub struct ListIterator {
    current: SingleLink,
}



impl Node {
    // A nice and short way of creating a new node
    pub fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None,
        }))
    }

    
}

impl BetterTransactionLog {
    pub fn new_empty() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        };
        self.length += 1;
        self.tail = Some(new);
    }
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            }else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
            .ok()
            .expect("Something is terribly wrong")
            .into_inner()
            .value
        })
    }
}

impl ListIterator {
    pub fn new(start_at: SingleLink) -> ListIterator {
        ListIterator {
            current: start_at,
        }
    }
}

impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None
        };
        result
    }
}


// Append entries at the end and remove entries from the front

/* Reverse
Using Rust's double ended Iterator.
Implementing this trait will provide the ability to reverse an iterator in a standardized way by offering
a next_back() function to get the previous value with the doubly linked list, that is only a matter of which
property gets set to the current item!
*/

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            },
            None => None
        };
        result
    }
}

/*
With this in place an iterator can be created by calling the iter() function on the list type, and by
calling iter().rev() the iterator will be reversed, providing the ability to go back as well as forward
*/


