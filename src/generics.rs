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