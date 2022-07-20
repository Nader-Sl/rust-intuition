// Book Ref : https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

use crate::util::*;
use crate::*; //Import the entire crate.

#[test]
pub fn box_type() {
    // A SmartPointer tp Point to Data on the Heap

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
    fn deref_coercion(big_obj: &BigStruct) {
        print_type_of("Type of big_object =", big_obj);
    }

    deref_coercion(&big_object);
    // Right after this scope exits, the BigStruct instance will be dropped and the memory related to it will be freed.
}

#[test]
pub fn rc_type() {
    // the Reference Counted Smart Pointer

    // In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
    // However, there are cases when a single value might have multiple owners.

    // The Rc<T> type keeps track of the number of references to a value to determine whether or not the value
    // is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

    // **Note that Rc<T> is only for use in single-threaded scenarios, Arc<T> is used instead for multi-threaded scenarios.**

    example_prologue!("rc_type");

    use std::rc::Rc;

    let owner_1 = Rc::new(String::from("Shared String"));
    println!(
        "Current ref counts for the Shared String = {}",
        Rc::strong_count(&owner_1)
    ); // prints 1
    {
        // cloning an RC only increases the ref count, it doesn't clone the wrapped value.
        let owner_2 = Rc::clone(&owner_1);
        println!(
            "Current ref counts for the Shared String is now {}",
            Rc::strong_count(&owner_2)
        ); // prints 2
    }
    // owner_2 RC goes out of scope and thus gets dropped, however because the contained string is
    // treated as a shared ownee, it doesn't get dropped, instead the reference count goes down back to 1.

    println!(
        "Current ref counts for the Shared String is now {}",
        Rc::strong_count(&owner_1)
    ); // prints 1
}

#[test]
pub fn refcell_type() {
    // The Interior Mutability Pattern

    // Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references
    // to that data; normally, this action is disallowed by the borrowing rules, but thos pattern uses unsafe code inside a data
    // structure to bend Rust’s usual rules that govern mutation and borrowing. ( refer to unsafe_ops.rs).
    // **Note : We cannot use 'mut' keyword to represent the interior mutability of a struct field.**

    // Lets create a MessageBox struct which holds a bunch of messages (Strings) in a vector
    // we should be able to push new messages to the message box.

    example_prologue!("refcell_type");
    struct BadMessageBox {
        messages: Vec<String>,
    }

    impl BadMessageBox {
        fn new() -> BadMessageBox {
            BadMessageBox {
                messages: Vec::new(),
            }
        }

        //This function enforces a mutable reference to the message box, so it can mutate the message box.
        fn push_message(&mut self, message: String) {
            println!("Msg : {}", message);
            self.messages.push(message);
        }
    }

    // In order for this to work, we are forced to set the whole BadMessageBox instance to mutable. which will
    // allow us to mutate any field within that struct, rendering it not that good of a practice.
    // try remove the 'mut' keyword and you will get a compilation error.

    let mut msg_box = BadMessageBox::new();
    msg_box.push_message(String::from("Hello World from BadMessageBox"));

    // We can use the RefCell<T> type to solve this problem, and allows us to mutate only the fields that are
    // wrapped by the RefCell smart pointer.
    use std::cell::RefCell;
    struct GoodMessageBox {
        messages: RefCell<Vec<String>>,
    }

    impl GoodMessageBox {
        fn new() -> GoodMessageBox {
            GoodMessageBox {
                messages: RefCell::new(Vec::new()),
            }
        }

        fn push_message(&self, message: String) {
            println!("Msg : {}", message);
            self.messages.borrow_mut().push(message);
        }
    }

    let msg_box = GoodMessageBox::new();
    msg_box.push_message(String::from("Hello World from GoodMessageBox"));
}
#[test]
pub fn custom_smart_pointer() {
    // We can create our own custom smart pointer type.
    // while std lib smart pointer types like Box, String, Vec etc.. are usually backed by a heap allocator to store the data
    // on the heap, this example however will wrap the data and always places it on the stack, however will demonstrate a typical
    // smart pointer architecture via Deref, DerefMut and Drop traits implementations.
    // PS: To see an example of a custom smart pointer backed by a heap allocation using libc, checkout unsafe_ops.rs.

    example_prologue!("custom_smart_pointer");

    use std::ops::{Deref, DerefMut, Drop};
    #[derive(Debug)]
    struct MySmartPointer<T> {
        data: T,
    }

    impl<T> MySmartPointer<T> {
        fn new(data: T) -> Self {
            MySmartPointer { data }
        }
    }
    impl<T> Deref for MySmartPointer<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            // defines the behavior of dereferencing the MySmartPointer type. (e.g. *my_ptr)
            &self.data
        }
    }
    impl<T> DerefMut for MySmartPointer<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            // defines the behavior of dereferencing the mut MySmartPointer type. (e.g. *my_mut_ptr)
            &mut self.data
        }
    }
    impl<T> Drop for MySmartPointer<T> {
        fn drop(&mut self) {
            // defines what happens when the MySmartPointer object is dropped (after its lifetime expires,
            // commonly after the containing scope exits). This is known as the RAII pattern in C++.

            // for now we will leave this empty.
            println!("Dropping MySmartPointer");
        }
    }

    // Although our smart pointer type places any generic object being passed to it on the stack,
    // if it happens to wrap a heap-allocator backed type object, like String like used below
    // the actual string data can still be moved onto the heap depending on the string SmallStringOptimization impl.

    let my_ptr = MySmartPointer::new(String::from("Hello"));

    print_type_of("Type of my_ptr = ", &my_ptr); //  MySmartPointer<String>

    print_type_of("Type of *my_ptr =", &(*my_ptr)); //  String

    let mut my_mut_ptr = MySmartPointer::new(String::from("Hello Mut"));

    // deref below calls into the 'deref' fn & yields an immutable reference to the wrapped String object.
    println!("my_mut_ptr value = {}", *my_mut_ptr); // Hello Mut

    // deref below calls into the 'deref_mut' fn & yields a mutable reference to the wrapped String object, which can be mutated.
    *my_mut_ptr = String::from("Modified Hello Mut");

    println!("my_mut_ptr value now is  = {}", *my_mut_ptr); // Modified Hello Mut

    // explicitly drop the pointers before the scope ends.

    drop(my_ptr);
    drop(my_mut_ptr);

    println!(
        "This is a proof that the pointers were dropped before the exit of this function scope"
    );
}
