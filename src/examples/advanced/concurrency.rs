// Book Ref : https://doc.rust-lang.org/book/ch16-00-concurrency.html

use crate::*; //Import the entire crate.
use std::thread;
use std::time::Duration;

#[test]
pub fn threading() {
    example_prologue!("threading");

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
    pub fn barrier_sync() {
        //TODO
    }

    #[test]
    pub fn mutex_sync() {
        //TODO
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
