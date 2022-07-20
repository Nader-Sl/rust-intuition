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

mod synchronization {

    #[test]
    pub fn message_passing() {
        //TODO
    }

    #[test]
    pub fn arc_sync() {
        //TODO
    }

    #[test]
    pub fn barrier_sync() {
        //TODO
    }

    #[test]
    pub fn condvar_sync() {
        //TODO
    }

    #[test]
    pub fn mutex_sync() {
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
