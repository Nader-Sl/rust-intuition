#[test]
pub fn vars_and_mut() {
    crate::example_prologue!("vars_and_mut");

    let x = 5; //immutable by default
               //x = 6; //compile error, its immutable.
    let mut y = 5; //marked as mutable, can be re-assigned.
    y += 6;
    println!("X = {}, y = {}", x, y);
}

#[test]
pub fn common_data_types() {
    crate::example_prologue!("common_data_types");

    //Don't worry about this function, it just prints the type of the passed param.
    pub fn print_type_of<T>(str: &str, _: &T) {
        println!("{} {}", str, std::any::type_name::<T>())
    }

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

#[test]
pub fn functions() {
    crate::example_prologue!("functions");

    //multiply the two floats passed in via a tuple param and returns the result.
    fn mul_function(param_x: (f32, f32)) -> f32 {
        let mul = {
            //scope based expression that decays into a value.
            let (x, y) = param_x;
            x * y
        };
        mul //returns mul, no semicolon here.
    }

    //Call the mul_function by passing in a tuple, and storing its result in a variable of type f32.
    let result = mul_function((3.3, 2.2));
    println!("mul_function call result: {}", result);
}

#[test]
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
