/*
The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:

Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
Attribute-like macros that define custom attributes usable on any item
Function-like macros that look like function calls but operate on the tokens specified as their argument
*/


//Macros can be a more advanced topic and will be covered few in example few chapters later..
//example_prologue is a 'declarative' macro used through out the examples to easily print information about a concept in a topic.
#[macro_export]
macro_rules! example_prologue {
 
    ($x:expr) =>  {
        println!("\n============================");
        println!("{} - {} ", file!(), $x);
        println!("============================\n");
    };
}