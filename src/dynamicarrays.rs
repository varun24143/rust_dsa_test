/*
Arrays - A fixed size container of length n, where every element has an equal size.
Thus any element can be reached by calculating the address to jump to using the basic understanding like
start_address + n * element_size
To make any array dynamic we will be using slices, which are views into a sequence data structure, akin to
an array. These are great fits when stored inside a Box pointer: allocated on the heap, it has all the benefits
of an array with a dynamic size
*/

type Node = Option<u64>;

pub struct DynamicArray {
    size: usize,
    cap: uszize,
    buf: Box<[Node]>,
}