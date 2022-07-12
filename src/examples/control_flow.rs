// Book Ref https://doc.rust-lang.org/book/ch03-05-control-flow.html
// Doc Refs:
// https://doc.rust-lang.org/stable/reference/expressions/loop-expr.html
// https://doc.rust-lang.org/stable/reference/expressions/range-expr.html
// https://doc.rust-lang.org/stable/reference/expressions/if-expr.html
// https://doc.rust-lang.org/stable/reference/expressions/match-expr.html

use std::option;

#[test]
pub fn if_else() {
    crate::example_prologue!("if_else");
    //The most common conditional branching among all languages.

    let number = 6;

    if number % 4 == 0 {
        //Notice that we don't need to surround the if expression with parentheses.
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

#[test]
pub fn if_in_let_statement() {
    crate::example_prologue!("if_in_let_statement");
    // Because if is an expression, we can use it on the right side of a let statement
    // to assign the outcome to a variable.
    let condition = true;
    let value = if condition { 10 } else { 0 };
    println!("value based on condition statement is {}", value);
}

#[test]
pub fn loop_keyword() {
    crate::example_prologue!("loop");
    // Loop keeps on looping indefinitely the inner content until it breaks or returns optionally.
    let mut loop_count: u32 = 0;
    loop {
        loop_count += 1;
        println!("iteration count {}", loop_count);
        if loop_count == 5 {
            //break the loop.
            return;
        };
    }
}

#[test]
pub fn while_loop() {
    crate::example_prologue!("while_loop");
    // While loop is similar to loop, but it has a condition.
    let mut loop_count = 0;
    while loop_count < 5 {
        loop_count += 1;
        println!("iteration count {}", loop_count);
    }
}

#[test]
pub fn for_loop() {
    crate::example_prologue!("for_loop");
    // For loop is similar to while loop, but it has a range.
    for i in 1..=5 {
        //this produces a range of numbers from 1 to 5, the '=' sign to mark the end range as inclusive.
        println!("iteration count = {}", i);
    }
}

#[test]
pub fn labeled_branches() {

    // We can label control flow branches which can be thought of as a handle so we can refer
    // to that particular tagged branch in nested branching if we want to modify the control flow
    // at that point.

    crate::example_prologue!("labeled_branches");

    'outer: for i in 1..=5 { // Iteration branch 1
        
        println!("iteration count = {}", i);

         for j in 1..=5 { //Iteration branch 2
            println!("iteration count = {}", j);
            if i == 4 && j == 2{
                // A plain break; here would only break the innermost loop ( iteration branch 2).
                println!("Breaking outer loop @ i = {} , j = {}", i, j);
                break 'outer; // breaks the outer loop ( iteration branch 1)
            }
        }
    }
}

#[test]
pub fn match_expressions() {
    crate::example_prologue!("match_expressions");
    //Similar to a case switch in other language, but way more powerful.

    use rand::Rng; //using Rng from rand crate (https://docs.rs/rand/0.8.5/rand/trait.Rng.html)
    let mut rng = rand::thread_rng(); // random generator
    let number = rng.gen_range(0..=20); // generated a random u32 [0..20]

    print!("Generated a random number {} => ", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match one of several values separated with an | operator.
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    crate::example_prologue!("match_guards");
    // Match guards allow you to check for additional conditions before executing the arm.

    let mut number: Option<u32> = Option::None; // number is a None value initially.

    let temp = rng.gen_range(0..=100); // generated a random u32 [0..100]

    if temp > 20 {
        // if temp is greater than 20, then set number to temp.
        number = Some(temp);
    };

    match number {
        Some(n) if n == 50 => println!("Got 50!"),
        Some(n) if n % 2 == 0 => println!("Got an even number {}", n),
        Some(n) => println!("Got a number {}", n),
        None => println!("Didn't get a number, temp must've been <= 20"),
    }
}

#[test]
pub fn if_and_while_let_expr() {
    // An if let expression is semantically similar to an if expression but in place of a condition
    // operand it expects the keyword let followed by a pattern, an = and a scrutinee operand.
    // If the value of the scrutinee matches the pattern, the corresponding block will execute.
    // Otherwise, flow proceeds to the following else block if it exists. Like if expressions,
    // if let expressions have a value determined by the block that is evaluated.

    let days = (
        // A tuple of days of the week.
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    );

    // We have destructured the tuple into variables, '_' is used to ignore the values we are not interested in printing.
    // We have declared a string literal "Tuesday" used for matching against the second element of the tuple..
    // The if let expression will first try to match the second element of the tuple against "Tuesday", if it matches then
    // the block will execute, otherwise it will be refuted and get skipped.

    if let (_, "Tuesday", x, _, _, _, _) = days {
        //Accepted since second element of the tuple is "Tuesday"
        println!("Day after Tuesday is {}", x);
    }

    if let (_, "Wednesday", x, _, _, _, _) = days {
        //Refuted since second element of the tuple is "Tuesday"
        println!("Day after Wednesday is {}", x);
    }

    // Same concept works with the while let expression.

    let mut stack = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // a stack of numbers

    // We have bound the range [5,10] to the variable 'n' via the @ symbol
    // and then we bound 'n' to a 'Some' unwrapped value (since pop() returns an Option and it could be None if the stack is empty).
    // we then inline it in a while let statement assigned to stack.pop()
    // What happens here is we first check if the popped top of the stack is in range [5,10], if it is then we print it,
    // and the loop continues in that manner and pops the next value off the stack. once the last popped value 'n' is out of the
    // range [5,10] the loop will break.
    
    while let Some(n @ 5..=10) = stack.pop() { 
        println!("n = {}", n);
    }
}
