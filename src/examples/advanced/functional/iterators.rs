//Book Ref: https://doc.rust-lang.org/book/ch13-02-iterators.html
//Doc Ref: https://doc.rust-lang.org/std/iter/trait.Iterator.html

// Processing a Series of Items with Iterators
// The iterator pattern allows you to perform some task on a sequence of items in turn. 
// An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
// In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

// Iterator is a trait that defines a method called next() that returns the next item in the sequence.
// Any struct that implements the Iterator trait can be used as an iterator. However there are plenty
// of general purpose containers that are already implement the trait for you in the standard library 
// like Vec, String, etc.

#[test]
pub fn iterators_basic() {
 //TODO
}


 