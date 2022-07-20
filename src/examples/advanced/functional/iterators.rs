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

use crate::*; //Import the entire crate.

mod fundementals {
    use super::*;
    struct Factory {
        products: Vec<String>,
    }

    impl Factory {
        fn new() -> Factory {
            Factory {
                products: vec![
                    String::from("Chocolate"),
                    String::from("Icecream"),
                    String::from("Soda"),
                ],
            }
        }
    }
    // There are three common methods which can create iterators from a collection:

    // iter(), which iterates over &T. (by ref)
    // iter_mut(), which iterates over &mut T. (by mut ref)
    // into_iter(), which iterates over T. (by value copy/move)

    #[test]
    pub fn iter() {
        example_prologue!("iter");

        let mut products = Factory::new().products;

        let mut iter = products.iter();

        while let Some(val) = iter.next() {
            // iterate over &T until products.iter.next() returns None (check control_flow.rs).
            println!("iter.next = : {}", val);
        }
        //an easier way to iterate over &T is via a for loop.
        for product in products.iter() {
            println!("product = : {}", product);
        }
    }

    #[test]
    pub fn iter_mut() {
        example_prologue!("iter_mut");
        // Iterate over &mut T to be able to change the value of it as we iterate.
        for product in Factory::new().products.iter_mut() {
            (*product) = product.to_uppercase(); //transform the product names to uppercase.
            println!("product = : {}", product);
        }
    }

    #[test]
    pub fn into_iter() {
        example_prologue!("into_iter");

        let products = Factory::new().products;

        // Iterating over T, since into_iter takes 'self' it will move the value 'products' to be
        // accessible only inside the for loop, after that we won't be able to use 'products'
        // because it got consumed by the into_iter method.
        for product in products.into_iter() {
            println!("product = : {}", product);
        }

        // Uncommenting the following line => compile time error, since 'products' is consumed and rendered unusable.
        // println!("products = : {:?}", products);

        // The simplest iteration over T is via a shorthand very simple for loop, which by internally calls into_iter().
        // Create a new Factory via the new factory method, because 'products' now is consumed by the iterator(perm moved).

        for product in Factory::new().products {
            //internally calls into_iter()
            println!("product = : {}", product);
        }

        // 'products' again is consumed at this point by the implicit into_iterator call and thus rendered no more usable.
    }
}

#[test]
pub fn iterator_adaptors() {
    example_prologue!("iterator_adaptors");
    // Iterator Adaptors allow you to change iterators into different kinds of iterators. You can chain multiple calls
    // to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have
    // to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

    let number_range = 0..10; //define a range [0,10[

    // We can convert a range to an iterator type and have it cast into a collection type. by explicitly assigning the
    // concrete collection type to the owning variable.

    let data: Vec<i32> = number_range.collect(); // we want to cast that collection to a Vec<i32>

    // Lets write a cool 1 liner using the iterator adaptors in combination with closures (check closures.rs)
    // to filter out all odd numbers from the range, map them into their power of 2 and finally sum them up.

    let result = data
        .into_iter() // Get a T iterator from the data collection.
        .filter(|x| x % 2 == 0) // Filter out all odd numbers, returning a new iterator.
        .map(|x| x.pow(2)) // Map each number into its power of 2, returning a new iterator.
        .sum::<i32>(); // Sum all the numbers in the iterator, returning a single value.

    println!("result = : {}", result);

    // At this point data is longer available since it were consumed by the into_iterator().
}

#[test]
pub fn custom_iterator() {
    // We can make an iterator out of any struct that implements the Iterator trait.
    // next() is the only required method for the iterator trait, we can add other optional methods.
    // Check out docs to see the various optional methods that you can override.

    // Let's create a StopWatch that takes in a limit (in seconds), and decrements the tick on every
    // iteration until it reaches 0 which marks the end of the iteration.
    example_prologue!("custom_iterator");
    struct StopWatch {
        limit: usize,
        tick: usize,
    }

    impl StopWatch {
        fn new(limit: usize) -> StopWatch {
            StopWatch { limit, tick: limit }
        }
    }
    // Implement the iterator trait so that we are able to use StopWath as an Iterator.
    impl Iterator for StopWatch {
        type Item = usize; //Item required by the Iterator trait.

        fn next(&mut self) -> Option<Self::Item> {
            if self.tick > 0 {
                // If current tick is > 0, decrement it and return it.
                // Decrement the tick.
                self.tick -= 1;

                Some(self.tick)
            } else {
                // else return None, marking the end of iteration.
                None
            }
        }
    }

    for tick in StopWatch::new(10) {
        //iterate implicity via into_iter().
        println!("Current Tick : {}", tick);
    }
}
