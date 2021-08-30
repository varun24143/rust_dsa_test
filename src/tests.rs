use std::cell::RefCell;
use std::rc::Rc;
use std::mem;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

// mod generics;

// mod memtesting;

// use crate::generics::*;
use crate::memtesting::MyStruct;
use crate::transactionlog::*;
use crate::conmut::*;

type Link = Rc<RefCell<Node>>;


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }

    // #[test]
    // fn test_transactionlog_empty() {
    //     let mut empty_log = Node::new(" ".to_string());
    //     assert_eq!(Link::new(std::cell::RefCell::(" ").to_string(), empty_log));
    // }

    #[test]
    fn test_string_sanity() {
        assert_eq!(Sanitize("abc def".to_string()), "abc_def");
    }

    #[test]
    fn test_struct_size() {
        assert_eq!(mem::size_of::<MyStruct>(), 3*mem::size_of::<u8>());
        //assert_eq!(mem::size_of::<MyStruct>(), 3*mem::size_of::<u8>());
    }

    // #[test]
    // fn open_door() {
    //     let mut door = Door::new(false);
    //     door.open();
    //     assert!(door.is_open);
    // }
}
