//Book Ref : https://doc.rust-lang.org/book/ch10-01-syntax.html#generic-data-types

// Generics provide us an effective solution to handle concept duplication without having to duplicate code needlessly.
// We use generics to create definitions for items like function signatures or structs, which we can then use
// with many different concrete data types.

pub fn manual_duplication() {

    crate::example_prologue!("manual_duplication");

    // We want to be able to have a functionality that will allow us return the largest
    // value in an array of 2 different types, i32 and char. The classic way to do this is
    // to write two separate yet redundant functions that is composed of the same body context
    // with the only difference being the type returned.

    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

pub fn generic_duplication() {

    crate::example_prologue!("generic_duplication");

    //Generics can make our life easier by providing us with a way to have
    //a single mutual implementation of a certain usage, and pass in the data type(s)
    //of our choice to be replaced with the generic type defined within the function itself.

    //**Good To Know:
    //  Generics are processed during the compile phase, Rust uses a Monomorphized generics approach.
    //  Briefly said, the compiler generates a different copy of the code of a generic function for each concrete
    //  type needed. For example, using a Vec<u64> and a Vec<String> in code will yield on the binary level two
    //  copies of the generated code for Vec: one for Vec<u64> and another for Vec<String>, but on source level
    //  it has the feel that you only have used the same function with a different meta param.
 

    //Here is the single function impl that can be used with any type with the trait bounds
    //std::cmp::PartialOrd + Copy (trait bounds can be checked in the ./traits.rs file)

    fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    //Notice that there's no need to explicitly declare the function template type,
    //the compiler will infer the right type via the param.

    let result = largest(&number_list);  
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

pub fn refs_lifetime_validation () {

 //Book Ref https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
// Rust Burrow checker is a powerful tool to prevent us from writing code that leads to dangling 
// references and what not, so the Burrow Checker must always be able to track the lifetime of a
// a memory reference/a burrowed, sometimes we need to be more declarative as coders to help
// the burrow checker figure out the rest on its own.

//Consider the following 'longest' function which returns the longest of two string slices.

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

//Uncommenting the anove  function will yield this error:
//"this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`"
   
// As mentioned earlier, Every reference in Rust has a lifetime, and the Burrow checker MUST be aware of it.
// in this case, this is a function that takes in any two string slices and returns any of them conditionally (dynamically).
// meaning that the Burrow Checker in this case isn't able to guess which arg is it going to return and we  also don't know
// the lifespans associated with the passed references, and therefore we need a mean to explicitly declare that.

//Here we are using what's called a 'generic lifetime parameter', passed in the same way we pass conventional 
//generic params except it starts with an apostrophe, indicating that it is a lifetime parameter. 
//In this way it will be pretty easy for the Burrow checker to validate if we do something stupid at compile time.

fn longest<'q>(x: &'q str, y: &'q str) -> &'q str {
    if x.len() > y.len() {  x } else { y }
}

let str1 = "Hello";
let str2 = "World!";

println!("The longest str is {}", longest(str1,str2));

}