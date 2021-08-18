/*
Concurrency and mutability
Concurrency means that parts of a program run independently of each other. 
Parallelism refers to these part executing at the same time.
For ease both these can be considered as concurrency itself
*/

use std::thread;// For threading
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Sender, Receiver}; 
use std::sync::{Mutex, Arc}; // Mutual Exclusion and Atomic Reference counterpub fn 

pub fn threading() {
    // the two pipes (||) where the parameters go, akin to a funcion signature's parameters,
    // without the need to always declare tpyes explicity. This was the variable can move from
    // the outer scope to inner scope 

    let handle = thread::spawn(move || {
        println!("Hello from a thread");
    });
    handle.join().unwrap();
}

pub fn Sanitize(s: String) -> String {
    let s = s.trim();
    let s = s.replace(" ", "_");
    s
}

/*
Interior Immutability
Can a variable be immutable and mutable at the same time ?
Ofcourse. Boxed variables (Box, Rc, and so on) are an immutable reference to the heap and they contain
the actual value
This concept of Interior Immutability is often used in combination with Rc in order to provide a value
to multiple owners with mutability at will.
Wrapping a RefCell in an Rc acts as the gatekeeper for having multiple owners, including a way to change
the contents. 
*/



type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

// impl Node {
//     pub fn append(&mut self, value: String) {
//         let new = Rc::new(RefCell::new(Node::new(value)));
//         match self.tail.take() {
//             Some(old) => {
//                 old.borrow_mut().next = Some(new.clone());
//                 new.borrow_mut().prev = Some(old); 
//             }
//             None => self.head = Some(new.clone()),
        
//         }
//     }
// }

/* Implementing channels in rust for passing multiple messages into a thread
or implementing an actor model, the Rust library offers channels. Channels are
single-conusmer multi-producer queues, that let the caller send messages to
multiple threads.  Below snippet spawns 10 threads and each thread sends a number
into the channel, where it will be collected into a vector after the senders
have finished executing 
*/



pub fn channels() {
    const N: i32 = 10;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let handles = (0..N).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            // don't use the result
            let _ = _tx.send(i).unwrap();
        })
    });
    // close all threads
    for h in handles {
        h.join().unwrap();
    }
    // receive N times
    let numbers: Vec<i32> = (0..N).map(|_| rx.recv().unwrap()).collect();
    println!("{:?}", numbers);
}

pub fn shared_state() {
    let v = Arc::new(Mutex::new(vec![]));
    let handles = (0..10).map(|i| {
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector = numbers.lock().unwrap();
            (*vector).push(i);
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", *v.lock().unwrap());
}

/*
Marker traits - fundamental to Rust's multithreading policies.
send - A data type is safe to send (move) from one thread to another
sync - The data type can be shared across threads without manual locks or mutex areas 
*/

use std::result::Result;
pub fn add(a: i32, b: i32) -> Result<i32, String> {
    let c = a + b;
    println!("{:?}", &c);
    Ok(c)
    
}

/*
Static and Dynamic 
Rust dependencies have 2 types of linking -
Static - Via the rlib format
Dynamic - Via shared libraries (.so or .dll)
For rust ideally dynamic linking is used for native dependencies, since they 
are usually available, in the operating system, and don't need to be included
in the package. Rust compiler favors this with a -C prefer-dynamic flag, which
will get the compiler to look for the corresponding dynamic libraries first.
*/