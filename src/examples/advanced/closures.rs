//Book Ref : https://doc.rust-lang.org/book/ch13-01-closures.html
//Doc Ref : https://doc.rust-lang.org/rust-by-example/fn/closures.html

// Closures: Anonymous Functions that Can Capture Their Environment
// Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
// You can create the closure in one place and then call the closure to evaluate it in a different context.
// Unlike functions, closures can capture values from the scope in which they’re defined/

#[test]
pub fn closure_basic() {
    let closure_fn = |x: i32| x + 1; // We store the closure in a variable.
    let y = closure_fn(5); // We call the closure.
    println!("{}", y); // We print the result.
}

#[test]
pub fn closure_capture() {

    // Closures can capture variables:
    // - by reference: &T
    // - by mutable reference: &mut T
    // - by value: T

    // They preferentially capture variables by reference and only go lower when required.

    let color_name = String::from("Green");
    let color_rgb = (0, 255, 0);
    let mut counter = 0;

    // The following closure captures color_name and color_rgb.
    // color_name doesn't implement the copy trait, so it can't be copied.
    // color_rgb is a tuple, it implements the copy trait only in that of arity 12 or less which applies 
    // only the copyable elements inside, so it can be entirely copied in our case as it consists of copyable integers.

    let mut capture_closure = || {
        println!("color name: {}", color_name);
        println!("color rgb : {}, {}, {} ", color_rgb.0, color_rgb.1, color_rgb.2);
        counter += 1;
        println!("counter : {}", counter);
    };

    // we are able here to drop the captured color_rgb before usage since we know it can be copied.
    // however if we don't call the following line the compiler can optimize it and avoid a copy
    std::mem::drop(color_rgb);  

    // We are not able to drop borrowed variables here as long as the capture_closure is still to be used.
    // Therefore we can't drop the color_name because it is not copyable to it needs to be burrowed.
    // We also can't drop the counter variable even if it is a copyabl integer because it is mutable and its 
    // global state needs to be reflected in the closure. therefore it is burrowed.

    // We call the closure.
    capture_closure();

}

#[test]
pub fn closure_move() {

    // Closures can also capture variables by move, which means they can move the ownership of the variables to the closure.
    // rendering these variables unavailable for use outside the closure.
    //This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread

    let numbers = vec![1, 2, 3];

    let handle = std::thread::spawn(move || {
        println!("Here's a vector of numbers: {:?}", numbers);q
    });

    //numbers can no longer be accessed here.

    handle.join().unwrap();
}
 