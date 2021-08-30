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