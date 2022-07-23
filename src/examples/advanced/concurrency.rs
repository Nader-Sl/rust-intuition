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
    use std::{borrow::BorrowMut, thread::JoinHandle};

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
            let barrier = Arc::clone(&barrier); //clone the Arc so it can be access by multiple threads.

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

        let stack_ref = Arc::new(Mutex::new(Vec::<String>::with_capacity(STACK_SIZE)));

        let mut thread_handles = Vec::<JoinHandle<_>>::with_capacity(2); // storage for the two threads handles.

        let stack_ref_ = Arc::clone(&stack_ref); //clone the Arc so it can be access by multiple threads.

        // Create a thread to push a value onto the stack every 10 milliseconds.
        thread_handles.push(thread::spawn(move || {

            for i in 0..STACK_SIZE {

                let str = "String#".to_string() + &i.to_string();

                // Acquire the lock on the stack, which will block the thread until the lock (underlying resource) is available.
                // We need to check if there's been a mutex poisioning caused by a panic while the stack lock is being held in another
                // thread, if so we can choose to panic here, return, or just continue, we'll just continue for now.

                let mut stack = match stack_ref_.lock() {
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

        let stack_ref_ = Arc::clone(&stack_ref); //clone the Arc so it can be access by multiple threads.

        // Create another thread to attempt to pop the values off the stack every 20 milliseconds.
        thread_handles.push(thread::spawn(move || {
            // Acquire the lock on the stack, which will block the thread until the lock (underlying resource) is available.
            for i in 0..STACK_SIZE {
                if let Some(str) = stack_ref_.lock().unwrap().pop() {
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
    #[allow(unused_must_use)]
    pub fn condvars() {
        // Conditional Variables.

        // Condition variables represent the ability to block a thread such that it consumes no CPU time while waiting for an event to occur.
        // condvars are typically associated with a boolean predicate (a condition) and a mutex. The predicate is always verified
        // inside of the mutex before determining that a thread must block.
        // Functions in this module will block the current thread of execution. Note that any attempt to use multiple mutexes on the same
        // condition variable may result in a runtime panic.

        //== Example ==
        // Let's take the example used in 'mutexes' test and modify it a little to demonstrate the condvars.
        // Suppose we have two threads, one that pushes onto and another that pops off a stack, this time
        // we only want to start popping off the stack once the pushing thread is done with its work.
        // Although this seems like we don't require the use of a mutex since in this way we can guarantee that
        // there won't be any data race condition having one thread write at a time, and the other read only after
        // the latter has finished writing, but we still have to use a mutex to access the interior mutability pattern
        // offered by a mutex, as the compiler will not allow the use of RefCell in multithreading context for extra safety.

        use std::sync::{Arc, Condvar, Mutex};

        let stack_ref = Arc::new(Mutex::new(Vec::<String>::new()));

        // We need 2 synchronization primitives for the cross-thread notification mechanism to work, a Mutex (done_mutex) and a Condvar (cvar).
        // The mutex will be used to wrap a bool (done) which will be set to true once the pushing thread is done. and the condvar will be used
        // to notify (awaken) all the blocked threads waiting to be notified to resume.
        let notify_prims_ref = Arc::new((Mutex::new(false), Condvar::new()));

        let mut thread_handles = Vec::<JoinHandle<_>>::with_capacity(2); // storage for the two threads handles.

        let stack_ref_ = Arc::clone(&stack_ref); //clone the Arc so it can be access by multiple threads.

        let notify_prims_ref_ = Arc::clone(&notify_prims_ref); //clone the Arc so it can be access by multiple threads.

        // Create a thread to push a value onto the stack.
        thread_handles.push(thread::spawn(move || {
            for i in 0..8 {
                // push 8 values onto the stack.

                let str = "String#".to_string() + &i.to_string();

                stack_ref_.lock().unwrap().push(str.clone()); // Now that the resource is free, push a string.

                println!("Pushed : {}", str);

                thread::sleep(Duration::from_millis(10)); // Sleep 10 millis between pushes.
            }

            // Now that the thread is done pushing values onto the stack, notify the popping thread that it can start popping values off the stack.

            // Destructure the done_mutex(Mutex) and cvar(Condvar) by reference (&) from dereferenced (*) notify_prims_ref_ (Arc).
            let (done_mutex, cvar) = &*notify_prims_ref_;
            let mut done = done_mutex.lock().unwrap();
            *done = true; // Set the done bool to true by re-assigning its dereferenced value.
            cvar.notify_one(); // Notify the popping thread that it can start popping values off the stack.
                               // ** we can also use cvar.notify_all() to notify all the blocked threads if we have more than one.
        }));

        let stack_ref_ = Arc::clone(&stack_ref); //clone the Arc so it can be access by multiple threads.

        let notify_prims_ref_ = Arc::clone(&notify_prims_ref); //clone the Arc so it can be access by multiple threads.
                                                               // Create a thread to pop the values off the stack.
        thread_handles.push(thread::spawn(move || {
            // Wait for the pushing thread to finish pushing values onto the stack without consuming unneeded cpu in a conventional loop.

            let (done_mutex, cvar) = &*notify_prims_ref_;
 
            // Wait for the pushing thread to finish pushing values onto the stack.
            cvar.wait(done_mutex.lock().unwrap());
            println!("The pushing thread seems to be done, let's pop the stack");
            while let Some(str) = stack_ref_.lock().unwrap().pop() {
                // Pop off the stack until there are no more values to pop.
                println!("Popped  {}", str);
            }
        }));

        // Wait for other threads to finish before returning the test by joining the stored handles.
        for handle in thread_handles {
            handle.join().unwrap();
        }
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

    //TODO
}
