//Don't mind this function, it is just to print the type of the passed param.
pub fn print_type_of<T>(str: &str, _: &T) {
    println!("{} {}", str, std::any::type_name::<T>())
}

pub fn vars_and_mut() {
    crate::example_prologue!("vars_and_mut");

    let x = 5; //immutable by default
               //x = 6; //compile error, its immutable.
    let mut y = 5; //marked as mutable, can be re-assigned.
    y += 6;
    println!("X = {}, y = {}", x, y);
}

pub fn common_data_types() {
    crate::example_prologue!("common_data_types");

    /*Almost all data types can be automatically infered by the compiler
    or even intellisense,however can be explicitly defined for precision.*/

    //Scalar types
    let _x: u8 = 1; //prefixing a var name with '_' marks it as deprecated so compiler won't warn if not used.
    let _x: i16 = 200;
    let _x: i32 = 200;
    let _x: i64 = 200;
    let _x: i128 = 200;
    let _x: isize = 200; //varies based on platform arch (32/64)
    let _x: f32 = 9.2;
    let _x = 2.0;
    let _x: bool = false;
    let x = 'c';

    print_type_of("type of scalar x =", &x);

    //strings
    let string_literal = "Hello World!"; //Hardcoded into the .text of the binary
    print_type_of("type of string_literal =", &string_literal);

    let string = String::from("Hello World!"); // Size not necessarily known at runtime, can be mutable, Held through a variable (runtime)
    print_type_of("type of string =", &string);

    /* Compound types */

    //Tuple
    let tup: (i16, bool, char) = (20, true, 'A');
    //Destructuring a tuple
    let (x, y, z) = tup;
    println!("Tuple Data : x = {}, y = {} , z = {}", x, y, z);

    //Array
    let arr = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; //repeat an element (3) x times (5)

    println!("arr = {:?}, arr2 = {:?}", arr, arr2); //pretty print 1
                                                    //println!("arr = {:#?}, arr2 = {:#?}", arr, arr2); //pretty print 2 (every component on a new line )
}

//multiply the two floats passed in via a tuple param and returns the result.
fn mul_function(param_x: (f32, f32)) -> f32 {
    let mul = {
        //scope based expression that decays into a value.
        let (x, y) = param_x;
        x * y
    };
    mul //returns mul, no semicolon here.
}

pub fn functions() {
    crate::example_prologue!("functions");

    let result = mul_function((3.3, 2.2));
    println!("mul_function call result: {}", result);
}

pub fn control_flow() {
    crate::example_prologue!("control_flow");

    //standard if-else handling
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if in a let statement, corresponds to a c++ ternary operator x ? y : z
    let condition = true;
    let value = if condition { 10 } else { 0 };
    println!("value based on condition statement is {}", value);

    //looping
    let mut loop_count = 0;
    //loop is short for while(true), it can be labeled or unlabaled, use labaled when in nested control flows to decide which to refer to.
    'loop_label: loop {
        loop_count += 1;
        println!("Loop count {}", loop_count);
        if loop_count == 5 {
            break 'loop_label;
        };
    }

    //looping over collection elements
    let arr = 1..=5; //this produces a range of numbers from 1 to 5, the '=' sign to mark the end range as inclusive;

    for e in arr.rev() {
        //.rev() reverses the range
        println!("Looping arr element : {}", e);
    }

    //Matching (Pattern Matching)
    //Similar to a case switch in other language, but way more powerful.

    use rand::Rng; //using Rng from rand crate (https://docs.rs/rand/0.8.5/rand/trait.Rng.html)
    let mut rng = rand::thread_rng(); // random generator
    let number = rng.gen_range(0..=20); // generated a random u32 [0..20]

    print!("Generated a random number {} =>", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }
}

pub fn var_shadowing() {
    crate::example_prologue!("var_shadowing");

    //Same scope shadowing.
    let x = 1;
    println!("x before shadowing = {}", x);

    //This will redefine var x similar to re-assigning it without 'let' but also without 'mut' and under a new owner.
    let x = 5;
    println!("x after shadowing = {}", x);

    //Inner scope Shadowing
    {
        //variable x can be defined inside its own scope, but it doesn't override the one in the outer scope
        //it is only bound to the lifetime of this scope.
        let x = 10;
        println!("x after inner scope shadowing = {}", x);
    } //scope ends, var x is now invalidated (popped out of the stack).

    // shadowing scope exited, variable x here refers to the value owner before the last inner scope.
    println!("x after inner shadowing scope exits = {}", x);
}
