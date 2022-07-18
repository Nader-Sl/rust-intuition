// Book Ref : https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

use crate::*; //Import the entire crate.

// To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code.
// You can take five actions in unsafe Rust, called unsafe superpowers, that you can’t in safe Rust.
//Those superpowers include the ability to:

// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

#[test]
pub fn main() {
    example_prologue!("main");

    // Using an FFI (Foreign Function Interface) is a great way to demonstrate unsafe code usage as we often have to dereference 
    // raw pointers  and call FFI functions. We will be using the libc crate (FFI binding) listed in the "Cargo.toml"
    // which allows us to natively allocate/deallocate and copy data directly on the heap.

    // In this example we are going to create our own smart pointer type that acts as a container
    // just like how String and Vec<T> are smart pointer based containers. We will create a FixedSizedStack
    // that lives in the heap allowing to push a huge sized data that otherwise isn't possible on the stack.
    // we will also implement the Deref trait which allows to dereference the FixedSizedStack by the '*' operator
    // and access the data it contains by reference, and the Drop trait which allows to free the memory on lifetime expiration.

    use libc::c_void; //https://crates.io/crates/libc

    struct FixedSizedStack<T, const N: usize> {
        // N is a constant generic parameter, you pass in a constant size.
        pointer: *mut T, // this is the raw mutable pointer to the memory allocated on the heap.
        curr_size: usize, //
    }

    // Implement the Drop trait to free the memory on lifetime expiration.
    impl<T, const N: usize> Drop for FixedSizedStack<T, N> {
        fn drop(&mut self) {
            println!("Freed the FixedSizedStack memory!");
            self.free();
        }
    }
    //implement the Deref trait for our struct so that we can dereference it by the '*' operator.
    impl<T, const N: usize> std::ops::Deref for FixedSizedStack<T, N> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe {
                //unsafe block required when dealing with raw pointers.
                let offset = self.curr_size - 1;
                self.pointer.add(offset).as_ref().unwrap() //As expected, it will panic if the reference is invalid.
            }
        }
    }

    impl<T, const N: usize> FixedSizedStack<T, N> {
        fn new() -> FixedSizedStack<T, N> {
            //factory method to create a new FixedSizedStack instance.
            unsafe {
                //unsafe block required when dealing with raw pointers.
                FixedSizedStack {
                    pointer: libc::malloc(std::mem::size_of::<T>() * N) as *mut T, // allocate memory on the heap that fits the fixed stack size.
                    curr_size: 0,
                }
            }
        }

        fn free(&mut self) -> bool {
            unsafe {
                if self.pointer == std::ptr::null_mut() {
                    return false;
                } // Guarantee no double freeing problems.
                libc::free(self.pointer as *mut c_void); //free the memory allocated on the heap.
                self.pointer = std::ptr::null_mut(); //set the pointer to null.
                self.curr_size = 0;
                true
            }
        }

        fn push(&mut self, value: *const T) {
            //push element raw pointer T on the stack, which can be passed in as a reference.

            if self.curr_size >= N {
                //bound checking
                println!("Failed to push, Stack is full!");
                return;
            }

            unsafe {
                //copy the value to the heap via native libc functions.
                //ps: Copy trait is irrelevant here because we are copying data of raw pointer.
                //so it doesn't matter if a struct that we are copying implements the copy trait.
                libc::memcpy(
                    (self.pointer.add(self.curr_size)) as *mut c_void,
                    value as *mut c_void,
                    std::mem::size_of::<T>(),
                );

                self.curr_size += 1; // increment size after pushing the element.
            }
        }

        fn pop(&mut self) -> *const T {
            // pop element T from the stack and return it as a raw pointer.

            if self.curr_size == 0 {
                //Bound checking
                println!("Failed to pop, Stack is empty!");
                return std::ptr::null(); //return nullpointer if the stack is empty.
            }
            unsafe {
                let offset = self.curr_size - 1;
                let res = self.pointer.add(offset); // get top of the stack
                self.curr_size -= 1; // decrement size after popping the element.
                res // return the popped element.
            }
        }

        fn empty(&self) -> bool {
            self.curr_size == 0
        }
    }

    const STACK_SIZE: usize = 10;
    //instantiate a new FixedSizedStack instance by factory method new.
    let mut stack = FixedSizedStack::<usize, STACK_SIZE>::new();

    for i in 1..=STACK_SIZE {
        // populate the stack with some data by iterating over the range 1..=STACK_SIZE.
        stack.push(&i);
        println!("Pushed {}", i);
    }

    println!("\n"); //newline

    while !stack.empty() {
        // iterate over the stack until it is empty.
        let e = stack.pop();
        unsafe {
            if e != std::ptr::null() {
                // only proceed if the element is not null.
                if stack.empty() {
                    println!("Popped {}, the stack is now empty.", *e);
                } else {
                    // if the stack is not empty, print the top of the stack element by dereferencing it
                    println!("Popped {}, curr item is {}.", *e, *stack);
                }
            }
        }
    }
}
