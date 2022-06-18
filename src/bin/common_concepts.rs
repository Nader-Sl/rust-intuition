

fn vars_and_mut() {
    println!("\nvars_and_mut");
    println!("-------------------------------------\n");
    let x = 5; //immutable by default
    //x = 6; //compile error, its immutable.
    let mut y = 5;//marked as mutable, can be re-assigned.
    y += 6;
    println!("X = {}, y = {}", x, y);
}

fn common_data_types() {

    println!("\ncommon_data_types");
    println!("-------------------------------------\n");
    /*Almost all data types can be automatically infered by the compiler 
    or even intellisense,however can be explicitly defined for precision.*/
    fn print_type_of<T>(str: &str, _: &T) {
        println!("{} {}", str, std::any::type_name::<T>())
    }
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
    let x  = 'c';

    print_type_of("type of scalar x =", &x);

    //strings
    let literal = "Hello World!";
    print_type_of("type of literal string =", &literal);

    /* Compound types */

    //Tuple
    let tup: (i16, bool, char) = (20, true, 'A');
    //Destructuring a tuple
    let (x, y , z) = tup;
    println!("Tuple Data : x = {}, y = {} , z = {}", x, y ,z);

    //Array
    let arr = [1,2,3,4,5];
    let arr2 = [3;5]; //repeat an element (3) x times (5)

    println!("arr = {:?}, arr2 = {:?}", arr, arr2); //pretty print 1
    //println!("arr = {:#?}, arr2 = {:#?}", arr, arr2); //pretty print 2 (every component on a new line )
}

//multiply the two floats passed in via a tuple param.
fn mul_function(param_x:(f32, f32)) -> f32{
 let mul = { //scope based expression that decays into a value.
     let (x,y) = param_x;
     x * y
 };
 mul //returns mul, no semicolon here.
}

fn control_flow() {

    println!("\ncontrol_flow");
    println!("-------------------------------------\n");
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
    let value = if condition {10} else {0};
    println!("value based on condition statement is {}", value);

    //looping
    let mut loop_count = 0;
    //loop is short for while(true), it can be labeled or unlabaled, use labaled when in nested control flows to decide which to refer to.
    'loop_label:loop { 
        loop_count+=1;
        println!("Loop count {}", loop_count);
        if loop_count == 5 {break 'loop_label};
    }

    //looping over collection elements
    let arr = 1..=5; //this produces a range of numbers from 1 to 5, the '=' sign to mark the end range as inclusive;

    for e in arr.rev() { //.rev() reverses the range
        println!("Looping arr element : {}", e);
    }
}

fn main() {
    vars_and_mut();
    common_data_types();
    control_flow();
    println!("\nfunction test");
    println!("-------------------------------------\n");
    println!("mul_function call result: {}",mul_function((3.3,2.2)));
 
}