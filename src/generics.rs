/*
Generics -
Supports generics and even allows to enforce the implementation of certain
traits. These constraints can come either come as a where clause attached to
the function definition or with a colon in the generic type of declaration.
*/

pub fn my_generic<T: MyTrait>(t: T) {
    // code
}

// this is same as

pub fn my_generic<T>(t: T) where T: MyTrait {
    // code
}

// but better use in 2018 and beyond

pub fn my_generic(t: impl MyTrait) {
    // code
}

/*
Accessing the Box
There is often a strong wish to put a reference to a trait inside a struct, rather
than directly working with the implementation.
To apply this architecture in Rust, the language requires us to put a trait's 
implmentation into a Box<dyn TheTrait>, making it more difficult handle, test,
and reason about. This trait object requires the compiler to rely on dynamic 
dispatch, which is considerably slower than the default static dispatch.
*/