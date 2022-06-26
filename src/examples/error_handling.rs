//Doc Ref https://doc.rust-lang.org/rust-by-example/error.html
pub struct Panic {
    //Unrecoverable Errors with panic!
    //Sometimes, bad things happen in your code, and there’s nothing you can do about it.
    //In these cases, Rust has the panic! macro. When the panic! macro executes, your program
    //will print a failure message, unwind and clean up the stack, and then quit. We’ll commonly
    //invoke a panic when a bug of some kind has been detected and it’s not clear how to handle
    //the problem at the time we’re writing our program.

    //Unwinding the Stack or Aborting in Response to a Panic
    //By default, when a panic occurs, the program starts unwinding, which means Rust walks
    //back up the stack and cleans up the data from each function it encounters. However,
    //this walking back and cleanup is a lot of work. Rust, therefore, allows you to choose
    //the alternative of immediately aborting, which ends the program without cleaning up.
    //Memory that the program was using will then need to be cleaned up by the operating system.
    //If in your project you need to make the resulting binary as small as possible, you can switch
    //from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile]
    //sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:
    //[profile.release]
    //panic = 'abort'
}

impl Panic {
    pub fn intentional() {
        //We can intentionally call panic! macro with a failure message.
        //Since we haven't change the default behaviour of panic in the cargo.toml file
        //panic will print  failure message, unwind and clean up the stack and then terminate the program.

        panic!("Unwind, Cleanup stack and Terminate.");
    }

    pub fn invalid_memory() {
        //referencing an invalid index (memory access violation) is unrecoverable & will lead to panicing!
        let v = vec![1, 2, 3];
        v[99];
    }
}

pub struct Result_Handling {
    // Most errors aren’t serious enough to require the program to stop entirely.
    // Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
    // For example, if you try to open a file and that operation fails because the file doesn’t exist,
    // you might want to create the file instead of terminating the process.

    // Functions that are prone to recoverable errors should return a Result object of the enum Result<T, E>.
    // We can then match the result returned against two possible outputs: 'Ok' and 'Err' which are
    // callback functions that allow us to handle each case separately.

    //Let's try to open a file in the current working directory, first without the file and then try create the file and retry.
}

use std::fs::File; // use the File struct in the fs module.
use std::io::{Error, ErrorKind}; //use error related types from the io module.

const FILE_NAME: &str = "sample.txt";

impl Result_Handling {
    pub fn panic_if_err_1() {
        //Try to open the file (returns a Result)
        let open_result = File::open(FILE_NAME);

        //Lets just panic for now in case the file wasn't found.
        let _resolved_file = match open_result {
            Ok(file) => file, //returns the file, everything is ok.
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }

    pub fn panic_if_err_2() {
        //More convinient 1 liner panic if error, call 'unwrap' on the result.
        File::open(FILE_NAME).unwrap();
    }

    pub fn panic_if_err_3() {
        //More convinient 1 liner panic if error with a custom message, call 'expect' on the result.
        let _resolved_file = File::open(FILE_NAME).expect("Failed to open sample.txt");
    }

    pub fn match_err_type() {
        //Let's now match the type of error and based on that take an action whether to try recover or panic.

        let open_result = File::open(FILE_NAME);

        let _resolved_file = match open_result {
            Ok(file) => file, // return the file, everything is ok.
            Err(error) => match error.kind() {
                ErrorKind::NotFound =>
                // File is not found, well let's create one then not a big deal (recover approach).
                {
                    match File::create(FILE_NAME) {
                        // now match another result, that of File::Create.
                        Ok(fc) => fc, //everything is ok, created a new file.
                        Err(e) => panic!("Problem creating the file: {:?}", e), //Darn, even file creation fail, hard to recover.
                    }
                }
                unknown_err => {
                    //This is another error type that "ErrorKind::NotFound", lets just panic.
                    panic!("Problem opening the file: {:?}", unknown_err)
                }
            },
        };
    }

    pub fn propagate_error() {
        // We can propagate errors wrapped in a Result to be handled by the caller.
        fn inner_propagate_error() -> Result<String, Error> {
            use std::io::Read; //required for read_to_string

            let open_result = File::open(FILE_NAME);

            let mut _resolved_file = match open_result {
                Ok(file) => file,
                Err(e) => return Err(e), // propagate error.
            };

            let mut s = String::new();

            match _resolved_file.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e), //propagate error.
            }
        }

        match inner_propagate_error() {
            Ok(_) => println!("All good"),
            Err(e) => eprintln!("Error : {}", e),
        }
    }

    pub fn custom_result() {
        //Lets now create our own Result returning function.

        //Consider the function that takes in only an unsigned even number and returns half of it
        //since even numbers are lways divisable by two. But we also want to make sure that
        //the parameter is an even number to return its half, otherwise resturn an errorenous result.

        fn half_even(even_number: u32) -> Result<u32, Error> {
            if even_number % 2 == 0 {
                Ok(even_number / 2)
            } else {
                //create a new error from ErrorKind and string.
                //Note that we can create our own Error type but will be easier to understand after covering 'Traits'.
                Err(Error::new(ErrorKind::InvalidInput, "Not an even number"))
            }
        }

        //Now lets create an Option based on the Result, so that if there is an error, we will return a None,
        //otherwise a sum wrapping the halfed value.

        let param = 3; //Changing this to an even number like 4 would yield a valid result.

        let result = match half_even(param) {
            Ok(n) => Some(n),
            Err(_e) => None,
        };

        println!(
            "Result : {}",
            match result {
                // The division was valid
                Some(x) => x.to_string(),
                // The division was invalid
                None => "Invalid".to_owned(),
            }
        )
    }
}
