// Book Ref : https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

use libc::c_void; //https://crates.io/crates/libc

use crate::*; //Import the entire crate.

#[test]
pub fn box_type() {
    // Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
    // But they don’t have many extra capabilities either. You’ll use them most often in these situations:

    // * When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // * When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // * When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

    example_prologue!("box_type");

    // Box type is a smart pointer that is used to store a value of a particular type on the heap, and a reference to it on the stack.
    // Lets try to use a Box to allocate a huge object on the heap instead of stack.
    #[derive(Debug)]
    struct BigStruct {
        data: [u32; 3000000],
    }

    // Note that in debug mode, the following code will first attempt to create the BigStruct instance on stack and then move
    // it to the heap. But since the stack size is limited, the following allocation will cause a stackoverflow error.
    // However in release mode, the code will succeed because the compiler is going to optimize it and allocate that buffer
    // directly on the heap, and therefore the code will compile.
    // You will need to run "cargo test" with the --release flag to get it to work. Alternatively if you are using the
    // the Rust-Analyzer extension to run this single test you need to add in the --release flag to the Runnables: Extra Args
    // config inside the extensions settings.

    let big_object = Box::new(BigStruct { data: [0; 3000000] });

    println!(
        "Allocated big_object of size {} on the heap!",
        big_object.data.len()
    );

    print_type_of("Type of big_object =", &big_object); //  Box<BigStruct>
    
    //We can dereference the box via "*" to access the object it wraps in the heap (BigStruct instance)
    print_type_of("Type of big_object =", &(*big_object)); // BigStruct

    //We can also access the object it wraps in the heap (BigStruct instance) via the "as_ref" method.
    print_type_of("Type of big_object =", big_object.as_ref()); // BigStruct

    // We also can utilize the deref coercion feature that Rust offers for functions which allows us
    // to pass a deref implementing object like Box<T> to a function that accepts its data by reference
    // allowing us to operate on the data without having to bother with the wrapper (e.g Box type).
    fn deref_coercion(big_obj : &BigStruct) {
       print_type_of("Type of big_object =", big_obj);
    }

    deref_coercion(&big_object);
    // Right after this scope exits, the BigStruct instance will be dropped and the memory related to it will be freed.
}

#[test]
pub fn custom_smartpointer() {
    example_prologue!("custom_smartpointer");

    // In this example we are going to create our own smart pointer type that acts as a container
    // just like how String and Vec<T> are smart pointer based containers. We will create a FixedSizedStack
    // that lives in the heap allowing to push a big number of data that otherwise isn't possible on the stack.
    // we will also implement the Deref trait which allows to dereference the FixedSizedStack by the '*' operator
    // and access the data it contains by reference, and the Drop trait which allows to free the memory on lifetime expiration.

    // We will be using the libc crate (FFI binding) listed in the "Cargo.toml" to allow us to allocate memory on the heap.
    // We will be also using unsafe code blocks to be able to use libc functions and tap into raw pointers which is by default
    // considered unsafe by rust since there are no strict guarantees to be made when working with that.

    struct FixedSizedStack<T, const N: usize> {
        // N is a constant generic parameter, you pass in a constant size.
        pointer: *mut T, // this is the raw mutable pointer to the memory allocated on the heap.
        curr_size: usize, //
    }

    // Implement the Drop trait to free the memory on lifetime expiration.
    impl <T,const N: usize> Drop for FixedSizedStack<T, N>  {
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

        fn free(&mut self) -> bool{
            unsafe {
                if self.pointer == std::ptr::null_mut() { return false} // Guarantee no double freeing problems.
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
