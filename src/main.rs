mod conmut;
mod memtesting;
mod transactionlog;

use crate::conmut::*;
use crate::memtesting::*;
use crate::transactionlog::*;

use std::mem;

struct Door {
    is_open: bool,
}

impl Door {
    fn new(is_open: bool) -> Door {
        Door {
            is_open: is_open,
        }
    }
}

trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_door() {
        let mut door = Door::new(false);
        door.open();
        assert!(door.is_open);
    }
}

use std::rc::Rc;

#[derive(Debug)]
pub struct FileName {
    name: Rc<String>,
    ext: Rc<String>,
}

pub fn ref_counter() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));

    for _ in 0..3 {
        println!("{:?}", FileName {
            name: name.clone(),
            ext: ext.clone(),
        });
    }
}

fn main() {
    println!("Hello, world!");
    let a = 50u8;
    let b = 50u128;
    println!("{:#?} {:#?}", a, b);
    ref_counter();
    conmut::threading();
    let read_str = conmut::Sanitize(String::from("abc def"));
    println!("{:#?}", read_str);
    conmut::channels();
    conmut::shared_state();
    println!("{:?}",add(5,4));
    // Testing memory assigned for the struct
    println!("{:?}",(mem::size_of::<MyStruct>(), 3*mem::size_of::<u8>()));
    println!("{:?}",(mem::size_of::<[MyStruct; 2]>(), 3*mem::size_of::<u8>()*2));

}

/* Declarative macros work on patterns and run code if that pattern matches
*/
#[macro_export]
macro_rules! vec {
    ($ ($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
        }
    };
}

