//Book Ref : https://doc.rust-lang.org/book/ch13-01-closures.html
//Doc Ref : https://doc.rust-lang.org/rust-by-example/fn/closures.html

// Closures: Anonymous Functions that Can Capture Their Environment
// Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
// You can create the closure in one place and then call the closure to evaluate it in a different context.
// Unlike functions, closures can capture values from the scope in which they’re defined/

use crate::*; //Import the entire crate.

#[test]
pub fn closure_basic() {
    example_prologue!("closure_basic");

    let closure_fn = |x: i32| x + 1; // We store the closure in a variable.
    let y = closure_fn(5); // We call the closure.
    println!("The closure function execution result = {}", y); // We print the result.
}

#[test]
pub fn closure_captures() {
    example_prologue!("closure_captures");
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
        println!(
            "color rgb : {}, {}, {} ",
            color_rgb.0, color_rgb.1, color_rgb.2
        );
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
pub fn closure_moves() {
    example_prologue!("closure_moves");
    // Closures can also capture variables by move, which means they can move the ownership of the variables to the closure.
    // rendering these variables unavailable for use outside the closure. This technique is mostly useful when passing a closure
    // to a new thread to move the data so it’s owned by the new thread.

    let numbers = vec![1, 2, 3];

    let handle = std::thread::spawn(move || {
        // add 'move' keyword before the pipes to have it in effect.
        println!("Here's a vector of numbers: {:?}", numbers);
    });

    //numbers can no longer be accessed here.

    handle.join().unwrap();
}

#[test]
pub fn closure_as_argument() {
    example_prologue!("closure_as_argument");
    // Closures can be used as arguments / parameters to functions.

    // When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few traits,
    // and they're determined by what the closure does with captured value. In order of decreasing restriction, they are:

    //  - Fn: the closure uses the captured value by reference (&T)
    //  - FnMut: the closure uses the captured value by mutable reference (&mut T)
    //  - FnOnce: the closure uses the captured value by value (T)

    // On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.
    // For instance, consider a parameter annotated as FnOnce. This specifies that the closure may capture by &T, &mut T, or T,
    // but the compiler will ultimately choose based on how the captured variables are used in the closure.

    fn open_mystery_box<F>(mystery_box_fn: F)
    where
        F: FnOnce(u32), // FnOnce being the most restrictive.
    {
        unsafe {
            // Unsafe is used here to allow the use of mutable static variables (counter).
            static mut COUNTER: u32 = 1;
            mystery_box_fn(COUNTER);
            COUNTER += 1;
        }
    }

    for _n in 0..10 {
        //loop 10 times.

        let announcement = "Opened Mystery Box #".to_owned();

        // By default the announcement is a non-copyable string and is referenced by the closure.
        // But inside the closure we call drop upon it, forcing it to be captured by value via a move.
        // Rendering it a must to set the open_mystery_box's closure function trait type to FnOnce.
        // Since we are going to be needing capture by value. But if we don't call drop on it we can
        // set the closure's trait type in the mystery_box_fn function to Fn, the less restrictive trait.
        let mystery_box_fn = |count: u32| {
            println!("{}{}", announcement, count);
            std::mem::drop(announcement);
        };

        open_mystery_box(mystery_box_fn);
    }
}
