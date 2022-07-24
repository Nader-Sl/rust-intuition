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
        example_prologue!("sync_primitives : Arc<T>");

        use std::sync::Arc;
        println!(
            "
         Atomically Reference Counted.

         The Arc<T> type is pretty much like the Rc<T> (check smart_pointers.rs) but instead uses
         atomic operations for its reference counting which is suitable for multi-threaded contexts.
         it requires using std::sync::Arc.,
         ** Use RC<T> for single threaded applications for a lower overhead.
        "
        )
    }

    #[test]
    pub fn weak() {
        example_prologue!("sync_primitives : Weak<T>");
        use std::sync::Weak;

        println!(
            "
        Atomic Weak type

        The std::sync::Weak<T> type is pretty much like the std::rc::Weak<T> (check smart_pointers.rs)
        but instead uses atomic operations for its reference counting which is suitable for multi-threaded
        contexts. **Use std::rc::Weak<T> for single threaded applications for a lower overhead.
        "
        );
    }

    #[test]
    pub fn barrier() {
        //Ensures multiple threads will wait for each other to reach a point in the program, before continuing execution all together.
        example_prologue!("sync_primitives : Barrier");

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
    pub fn mutex() {
        //A mutual exclusion primitive useful for protecting shared data

        // This mutex will block threads waiting for the lock to become available. The mutex can also be statically initialized
        // or created via a new constructor. Each mutex has a type parameter which represents the data that it is protecting.
        // The data can only be accessed through the RAII guards returned from lock and try_lock, which guarantees that the data
        // is only ever accessed when the mutex is locked.

        // == Example ==
        // Suppose we have two threads, one is pushing a value into the stack and the other is popping the last value onto the stack.
        // at a difference pace, In order to guarantee thread safety, we can wrap the stack (vector) with a mutex to ensure that only
        // one thread can access it at a time and guarantee a data-race free operation.

        example_prologue!("sync_primitives : Mutex<T>");
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
    pub fn condvar() {
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

        example_prologue!("sync_primitives : Condvar");

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
    pub fn once() {
        // A synchronization primitive which can be used to run a one-time global initialization.
        // Useful for one-time initialization for FFI or related functionality. This type can only be constructed with Once::new().

        example_prologue!("sync_primitives : Once");

        //== Example ==
        use std::panic;
        use std::sync::Once;

        static INIT: Once = Once::new(); //Doesn't require to be wrapped by an Arc to share among threads.

        let mut thread_handles = vec![];

        // Run 5 threads, each calls the initialization function.
        for i in 1..6 {
            thread_handles.push(thread::spawn(move || {
                INIT.call_once(|| {
                    // only one thread will get to call this.
                    println!("call_once() << Called INIT from thread# {}.", i); // printed only once.
                })
            }));
        }

        // Once can be poisoined with panics just like mutexes, When the initialization function panics via call_once
        // it will not be able to retry initialization again to try and recover, to handle this we can use the call_once_force()
        // which gives it another chance to retry the init until it succeeds. Once it does, both call_once and call_once_force()
        // become non-op since it already succeeded and that was the goal, otherwise we can call_once_force() until it succeeds.

        static INIT_: Once = Once::new(); //Doesn't require to be wrapped by an Arc to share among threads.

        // This is a panic hook to supress the default verbose stacktrace output print, make it just print the message instead.
        panic::set_hook(Box::new(|_info| {
            println!("{}", _info.to_string());
        }));

        // Run 5 threads, each calls the initialization function.
        for i in 1..6 {
            thread::sleep(Duration::from_millis(20)); // Sleep 20 millis between pushes.

            thread_handles.push(thread::spawn(move || {
                INIT_.call_once_force(|state| {
                    if i > 1 {
                        println!(
                            "call_once_force() << last OnceState of thread# {} is {}",
                            i - 1,
                            if state.is_poisoned() {
                                "Poisoned"
                            } else {
                                "Ok"
                            }
                        );
                    }
                    if (1..3).contains(&i) {
                        // Let's get the first and second threads to panic.
                        panic!("call_once_force() << Panic in INIT in thread# {}", i);
                    } // panic in INIT.

                    println!("call_once_force() << Called INIT from thread# {}", i);
                    // printed only once, on third thread
                })
            }));
        }

        // Wait for other threads to finish before returning the test by joining the stored handles.
        for handle in thread_handles {
            let res = handle.join();
            // match the join to force the test pass.
            match res {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    #[test]
    pub fn rwlock() {
        // This type of lock allows a number of readers or at most one writer at any point in time.
        // The write portion of this lock typically allows modification of the underlying data (exclusive access)
        // and the read portion of this lock typically allows for read-only access (shared access).

        // The priority policy of the lock is dependent on the underlying operating system’s implementation,
        // and this type does not guarantee that any particular policy will be used. In particular, a writer which
        // is waiting to acquire the lock in write might or might not block concurrent calls to read etc..

        //**Note : As a general rule, use an RWlock only in cases where there are so many threads that are concurrently
        // reading a shared piece of data without any or with only one writer thread, this will improve the concurrency of
        // the reads which can weigh enough to care about. otherwise use a Mutex as it is a less complex object with less
        // constraints and more guarantees. */
        example_prologue!("sync_primitives : Rwlock");

        //== Example ==
        // Lets spawn some threads to read a vector of strings
        // as only one writer thread is constantly pushing data to it.

        use std::sync::{Arc, RwLock};

        const READER_THREADS_N: usize = 3;

        const DATA_SIZE: usize = 8;

        let mut thread_handles = Vec::<JoinHandle<_>>::with_capacity(READER_THREADS_N + 1); // +1 for the writer thread.

        // Create a new RwLock around a vector for the numbers to be pushed to by the writer thead and read concurrently by reader threads.
        let rwlock_ref = Arc::new(RwLock::new(Vec::<usize>::with_capacity(DATA_SIZE)));

        let rwlock_ref_ = Arc::clone(&rwlock_ref); //clone the Arc so it can be access by multiple threads.

        thread_handles.push(thread::spawn(move || {
            for i in 0..DATA_SIZE {
                let mut writer = rwlock_ref_.write().unwrap();
                writer.push(i);
                println!("Writer Thread Pushed : {}", i);
            }
        }));

        for i in 0..READER_THREADS_N {
            let rwlock_ref_ = Arc::clone(&rwlock_ref); //clone the Arc so it can be access by multiple threads.
            thread_handles.push(thread::spawn(move || {
                let reader = rwlock_ref_.read().unwrap();
                // filter out the odd numbers.
                let collected = reader
                    .iter()
                    .filter(|&x| x % 2 == 0)
                    .collect::<Vec<&usize>>();
                println!("Reader Thread # {} collected : {:?}", i, collected);
            }));
        }
        for handle in thread_handles {
            handle.join().unwrap();
        }
    }
}

#[test]
pub fn mpsc() {
    example_prologue!("sync_primitives : mpsc");
    // Rust offers the MPSC (Multi-Producer Single-Consumer) model for message/data passing between threads
    // in which there can be multiple producers (or transmitters) and only one consumer (or receiver).
    // This model is internally based on a mpsc_queue that implements internal Atomic sync primitives to
    // ensure thread safety.

    //TODO
}
