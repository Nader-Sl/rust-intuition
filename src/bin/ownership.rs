#![allow(unused_variables)]
/* ------- Ownership Rules -------
* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.*/

fn assignment_ownership() {
    /* Simple types of a fixed size known at compile time such as the scalar types which are entirely living on
    the stack are always copied/cloned on assignment, such types implement the Copy trait and that includes:
        * All the integer types, such as u32.
        * The Boolean type, bool, with values true and false.
        * All the floating point types, such as f64.
        * The character type, char.
        * Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    */

    println!("\nassignment_ownership");
    println!("-------------------------------------\n");

    let x = 5; //x owns the data (5)
    let y = x; // y owns a copy of x's data (5) since i32 is a simple type that implements the copy trait.
    println!("x = {}, y = {}", x, y);

    //Complex types ownership has more complexity to it, by default, assignment moves the ownership from one variable to the other.
    let s1 = String::from("hello"); //s1 owns the String type "hello" (this is not a string literal type)
    let s2 = s1; // s1 ownership is transfered to the var s2 rendering s1 invalid.

    println!("s2 = {}", s2);
    //using s1 at this point will cause a compile-time error, uncomment the following line to try it:
    //println!("s1 = {}", s1);

    //if we want to perform a deep copy of s1 into s2, we should call the clone function instead.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn function_ownership() {
    println!("\nfunction_ownership");
    println!("-------------------------------------\n");

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!(
            "String \"{}\"'s ownership was transfered to takes_ownership function",
            some_string
        );
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!(
            "cloned i32 ({})'s and transfered ownership of that clone to makes_copy function",
            some_integer
        );
    } // Here, some_integer goes out of scope. Nothing special happens.

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's ownership is transfered to the function call and therefore it gets dropped after the call.

    //Uncommenting the following line will result in a compilation error.
    //println!("s = {}", s);

    let x = 5; // x comes into scope

    makes_copy(x); // x's clone ownership is transfered to the function call, x's ownership is still intact.

    println!("x's ownership remains intact, x = {}", x);

    fn gives_ownership() -> String {
        // gives_ownership will move its return value into the function  that calls it

        let some_string = String::from("yours"); // some_string comes into scope

        some_string // some_string is returned and moves out to the calling function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope
        a_string // a_string is returned and moves out to the calling function
    }

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    println!("s1 = {}, s3 = {}", s1, s3);
    //Uncommenting the following line will result in a compilation error because s2's ownership has been transfered to the 'takes_and_gives_back function and then onto s3.
    //println!("s2= {}", s2);
}

fn refs_and_burrowing() {
    /*
        If we want to access the data owned by another variable without having to transfer ownership, we can use references.
        Rules of references:
            * references are immutable by default so we can't change their value.
            * references can be set to mutable with one big restriction: you can have only one mutable reference to a particular piece of data at a time. 
     */
    println!("\nrefs_and_burrowing");
    println!("-------------------------------------\n");

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("hello");

    let len = calculate_length(&s1); // we pass the reference of s1 denoted by '&' preceeding the var name.

    //Since we only passed the reference of s1 to the 'calculate_length' func and didn't transfer the ownership of it, we are still able to use it.
    println!("The length of '{}' is {}.", s1, len);

    /* Mutable references */

    let mut s = String::from("hello");

    let ms1 = &mut s;

    ms1.push_str(" world!"); // append " world!" to s's data.
    
    println!("ms1 = {}", ms1);

    let ms2 = &mut s;

    //Uncommenting the following line will result in a compilation error because there can only be one mutable reference to the same piece of data at a time.
    //println!("ms1 = {}", ms1);
    
}


fn dangling_reference() {

    /* It is impossible for the compiled code to have dangling references, the compiler will always catch it at compile phase*/
    
    //Uncommenting the following function will throw a compile-time error.
    // fn dangle() -> &'static String {
    //     let s = String::from("hello");
    //     &s
    // }  
}

fn main() {
    assignment_ownership();
    function_ownership();
    refs_and_burrowing();
    dangling_reference();
}
