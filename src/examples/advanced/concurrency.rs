// Book Ref : https://doc.rust-lang.org/book/ch16-00-concurrency.html

use crate::*; //Import the entire crate.
use std::thread;
use std::time::Duration;

#[test]
pub fn threading() {
    example_prologue!("threading");
    // == Example ==
    // We will create a threaded announcer that pops the next announcement from a Vec of announcements
    // and prints it every 3 seconds, we concurrently wait on the thread till it finishes to return this function.

    let mut announcements = vec![
        "Hello Rust Fans!",
        "Rust is an amazing lang!",
        "Have a good day!",
    ];

    // We can prepend the thread closure with the 'move' keyword which allows us to move the announcement into the
    // closure. rendering it unusable thereafter.

    let thread_handle = thread::spawn(move || {
        // returns a JoinHandle which can be used to join this thread.
        while !announcements.is_empty() {
            // keep popping announcements until the Vec is empty.
            println!("{}", announcements.pop().unwrap());
            thread::sleep(Duration::from_secs(1)); //sleep for 1 seconds.
        }
    });

    //'announcements' is no longer accessible here since its been moved into the thread's closure.

    thread_handle.join().unwrap(); // wait for the thread to finish before we return this test.
}

mod sync_primitives {
    use std::thread::JoinHandle;

    use super::*;

    #[test]
    pub fn arc() {
        // Atomically Reference Counted.

        // The Arc<T> type is pretty much like the Rc<T> (check smart_pointers.rs) but instead uses
        // atomic operations for its reference counting which is suitable for multi-threaded contexts.
        // ** Use RC<T> for single threaded applications for a lower overhead.
        use std::sync::Arc;
    }

    #[test]
    pub fn weak() {
        // Atomic Weak type

        // The std::rc::Weak<T> type is pretty much like the std::sync::Weak<T> (check smart_pointers.rs)
        // but instead uses atomic operations for its reference counting which is suitable for multi-threaded
        //contexts. **Usestd::rc::Weak<T> for single threaded applications for a lower overhead.

        use std::sync::Weak;
    }

    #[test]
    pub fn barriers() {
        //Ensures multiple threads will wait for each other to reach a point in the program, before continuing execution all together.

        use std::sync::{Arc, Barrier};

        const THREADS_N: usize = 5; // define the number of threads to use.

        // create a JoinHandle vector to store the threads hands on spwan so that we can use them to join them later.
        let mut thread_handles = Vec::with_capacity(THREADS_N);

        // Create a THREADS_N size'd barrier to operate on THREADS_N threads, wrap the barrier with an Arc to make it thread safe.
        let barrier = Arc::new(Barrier::new(THREADS_N));

        for _ in 0..THREADS_N {
            let barrier = Arc::clone(&barrier); //clone the Arc to increase the strong ref count.

            thread_handles.push(thread::spawn(move || {
                println!("Hello Rust Fans!");
                println!("Barriers are great!");

                // Prints the first two messages probably interleavingly amongst the threads.

                barrier.wait(); // Waits for all threads to syncrhonize at this point before catching up with what follows.

                println!("know you know how barriers work."); // Prints after all threads have sync'd without any interleaving.

                barrier.wait(); // Waits for all threads to syncrhonize at this point before catching up with what follows.

                println!("Have a good day!"); // Prints after all threads have sync'd without any interleaving.
            }));
        }
        // Wait for other threads to finish before returning the test by joining the stored handles.
        for handle in thread_handles {
            handle.join().unwrap();
        }
    }

    // The following test 'mutexes' requires either removing the --release flag from the test command line
    // or alternatively choose to 'Debug' instead of running as test (available via Rust-Analyzer).
    // The poisoined mutex handling feature won't work in a release test mode.
    #[test]
    pub fn mutexes() {
        //A mutual exclusion primitive useful for protecting shared data

        // This mutex will block threads waiting for the lock to become available. The mutex can also be statically initialized
        // or created via a new constructor. Each mutex has a type parameter which represents the data that it is protecting.
        // The data can only be accessed through the RAII guards returned from lock and try_lock, which guarantees that the data
        // is only ever accessed when the mutex is locked.

        // == Example ==
        // Suppose we have two threads, one is pushing a value into the stack and the other is popping the last value onto the stack.
        // at a difference pace, In order to guarantee thread safety, we can wrap the stack (vector) with a mutex to ensure that only
        // one thread can access it at a time and guarantee a data-race free operation.

        use std::sync::{Arc, Mutex};

        // Create a Mutex to guard a vector of strings of cap = STACK_SIZE for synching over the shared data.
        // and then wrap the Mutex itself with an Arc to have its ownership shared amongst multiple threads.

        const STACK_SIZE: usize = 10;

        let thread_safe_stack = Arc::new(Mutex::new(Vec::<String>::with_capacity(STACK_SIZE)));

        let mut thread_handles = Vec::<JoinHandle<_>>::with_capacity(2); // storage for the two threads handles.

        let stack = Arc::clone(&thread_safe_stack); //clone the Arc to increase ref count so its not dropped on move.

        // Create a thread to push a value onto the stack every 10 milliseconds.
        thread_handles.push(thread::spawn(move || {
            for i in 0..STACK_SIZE {

                let str = "String#".to_string() + &i.to_string();

                // Acquire the lock on the stack, which will block the thread until the lock (underlying resource) is available.
                // We need to check if there's been a mutex poisioning caused by a panic while the stack lock is being held in another
                // thread, if so we can choose to panic here, return, or just continue, we'll just continue for now.

                let mut stack = match stack.lock() {
                    Ok(guard) => guard, // we just return the guard.
                    Err(poisoned) =>  {
                        // Poisioned mutex handling.
                        println!("The popping thread seems to have panicked! but we can continue pushing new values on to the stack.");
                        poisoned.into_inner() // calling into_inner will just ignore the mutex poisioning and continue its execution.
                    },
                };
                stack.push(str.clone()); // Now that the resource is free, push a string.
                println!("Pushed : {}", str);
                thread::sleep(Duration::from_millis(10)); // Sleep 10 millis between pushes.
            }
        }));

        let stack = Arc::clone(&thread_safe_stack); //clone the Arc to increase ref count so its not dropped on move.

        // Create another thread to attempt to pop the values off the stack every 20 milliseconds.
        thread_handles.push(thread::spawn(move || {
            // Acquire the lock on the stack, which will block the thread until the lock (underlying resource) is available.
            for i in 0..STACK_SIZE {
                if let Some(str) = stack.lock().unwrap().pop() {
                    if i == 2 {
                        // We intentionally panic on 2nd iteration to test the mutex posioning handler in the pushing thread.
                        panic!("The Mutex is now Posioned!");
                    }
                    println!("Popped  {}", str);
                }

                thread::sleep(Duration::from_millis(20)); // Sleep 20 millis between attempted pops.
            }
        }));

        // Wait for other threads to finish before returning the test by joining the stored handles.
        for handle in thread_handles {
            handle.join().unwrap_or_else(|e| {
                println!("One of the threads has panicked!"); // We expect this to be printed as the popping thread panics.
            });
        }
    }

    #[test]
    pub fn condvar_sync() {
        //TODO
    }
    #[test]
    pub fn once_sync() {
        //TODO
    }

    #[test]
    pub fn rwlock_sync() {
        //TODO
    }
}

#[test]
pub fn message_passing() {
    // Rust offers the MPSC (Multi-Producer Single-Consumer) model for message/data passing between threads
    // in which there can be multiple producers (or transmitters) and only one consumer (or receiver).
    // This model is internally based on a mpsc_queue that implements internal Atomic sync primitives to
    // ensure thread safety.
}
