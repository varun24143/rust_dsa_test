/*
The best transaction log
To improve the transaction log in the way the product team describes, it's a perfect fit for a skip list
Ordering the commands by a u32 number - a millisecond offset from the initial timestamp. The command it contains
are going to be stored as strings associated with the offset
In this the major difference is that the next pointer is going to be an array, which is due to the node
having a different successor at every level.
Secondly the content which was previously named as value will be named as offset here to differentiate between
the timestamp offset and the actual content
*/

use std::cell::RefCell;
use std::rc::Rc;
use rand::random;

pub type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
pub struct Node {
    pub next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

/*
Now we will implement a parameter that will reflect the size of the list and the highest level only contains
2 or three nodes at most
*/

#[derive(Clone)]
pub struct BestTransactionLog {
    head: Link,
    tails: Vec<Link>, // This property is a vector pointing to the tail of each level
    max_level: usize,
    pub length: u64,
}

// Adding Data
/*
A skip list only works if the values are somehow comparable and follow an ascending order. Easy to understand
skipping only makes sense if you know where you are going and you are moving ahead.
*/

impl BestTransactionLog {

    pub fn new_empty(max_level: usize) -> BestTransactionLog {
        BestTransactionLog {
            max_level: max_level,
            head: None,
            length: 0,
            tails: vec![None; max_level+1],
        }
    }

    pub fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n<self.max_level {
            n += 1;
        }
        n
    }
    
    pub fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level() // determine the levels by coin flip
        };
    }
}
