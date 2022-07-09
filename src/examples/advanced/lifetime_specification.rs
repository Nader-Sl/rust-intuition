// Book Ref https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// Doc Refs
// https://doc.rust-lang.org/stable/reference/trait-bounds.html
// https://doc.rust-lang.org/stable/reference/lifetime-elision.html

// Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
// Rust Burrow checker is a powerful tool to prevent us from writing code that leads to dangling
// references and what not, so the Burrow Checker must always be able to track the lifetime of a
// a memory reference/a burrowed. Sometimes however it might be too complex or very runtime-oriented
// for the burrow checker to have a handle on the lifetime expectation of references, and for such
// scenarios we have to be more declarative with our code and specify lifetimes relations via
// 'annotations syntax' and 'generic lifetime parameters'
#[test]
pub fn function_lifetime_generics() {

    crate::example_prologue!("function_lifetime_generics");

    // Consider the following function  'longest' which returns the longest of two string slices.

    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // }

    // Uncommenting the above function will yield this error:
    // "this function's return type contains a borrowed value, but the signature does not say whether
    // it is borrowed from `x` or `y`"

    // As mentioned earlier, Every reference in Rust has a lifetime, and the Burrow checker MUST be aware of it.
    // in this case, this is a function that takes in any two string slices and returns any of them conditionally (dynamically).
    // meaning that the Burrow Checker in this case isn't able to guess which arg is it going to return and we  also don't know
    // the lifespans associated with the passed references, and therefore we need a mean to explicitly declare that.

    // Here we are using what's called a 'generic lifetime parameter', passed in the same way we pass conventional
    // generic params (./generics.rs) except it starts with an apostrophe, indicating that it is a lifetime parameter.
    // then we are using the 'annotation syntax' in the functions's parameters to bind them to the same lifetime.
    // In this way it will be pretty easy for the Burrow checker to validate if we do something stupid at compile time.

    fn longest<'q>(x: &'q str, y: &'q str) -> &'q str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let str1 = "Hello";
    let str2 = "World!";

    // Burrow checker here sees that all the lifetime'd refs in the longest function have the same lifetime
    // since they are in the same scope, so it compiles normally.
    println!("The longest str is {}", longest(str1, str2));

    let mut result: &str = "Uncomment the following line of code to see an error.";
    {
        let string1 = String::from("Hello");

        // Uncommenting the line of code below will yield an error (`string1` does not live long enough).
        // The Burrow checker will notice in the following statement that str2 outlives string1 as a string slice
        // because they are in different scopes of different lifetimes.

        //result = longest(string1.as_str(), str2);
    }
    println!("The longest str is {}", result);
}

#[test]
pub fn struct_lifetime_generics() {

    crate::example_prologue!("struct_lifetime_generics");

    // We can define structs to hold references, but in that case we would need to add a lifetime annotation.

    // A Text struct that takes in a generic lifetime parameter wraps a lifetime annotated string slice.
    struct Text<'a> {
        content: &'a str,
    }

    impl<'a> Text<'a> { // This is how the lifetime parameters is defined in the impl block.
        fn print(&self){
            println!("Content = {}", self.content);
        }
    }
    let text = Text {
        content: "Hello World!",
    };

    text.print();
}

#[test]
pub fn static_lifetime() {

    crate::example_prologue!("static_lifetime");

    // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live
    // for the entire duration of the program.
    // The text of this string is stored directly in the program’s binary, which is always available.
    // Therefore, the lifetime of all string literals is 'static.

    let _str: &'static str = "I have a static lifetime.";
    println!("{}", _str);
}

#[test]
pub fn lifetime_elision() {
    
    crate::example_prologue!("lifetime_elision");

    // The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.
    // The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler
    // gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler
    // will stop with an error. These rules apply to fn definitions as well as impl blocks.

    // Three steps take effect for the compiler to decide the eligibility of lifetime annotation elision.

    // 1) the compiler assigns a lifetime parameter to each parameter that’s a reference.
    //    In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
    //    a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); etc..

    // 2) If there is exactly one input lifetime parameter, that lifetime is assigned to all
    //   output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

    // 3) If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
    //   the lifetime of self is assigned to all output lifetime parameters.

    // Below step 1 & 2 are in effect and thus are enough to make this function eligible for lifetime annotation elision
    fn one_input(ret: &str) -> &str {
        ret
    }

    println!("one_input result = {}", one_input("Hello World!"));

    // In the below commented function, step 1 is in effect, step 2 doesn't comply because there is more than one input, and step 3 doesn't comply either
    // because there is no 'self' as a parameter because it is a function that doesn't below to a struct (is not a method).

    // commenting out the following function will yeild a compilation error.
    // fn multiple_inputs_no_self(a: &str, b: &str) -> &str {
    //     return a
    // }


    //////// Now lets see how we can make use of the 3rd step/rule to write less verbose code ////////

    struct Manual<'a> {  // Check out the 'struct_lifetime_generics' example
        page_1: &'a str,
        page_2: &'a str,
        page_3: &'a str,
    }

    impl<'a> Manual<'a> {  

        fn default() -> Manual<'a>{ //Just a defult function to easily retrieve a factory defaulted object.
            Manual{
                page_1 : "empty_page_1",
                page_2 : "empty_page_2",
                page_3 : "empty_page_3"
            }
        }

        // The function bellow is eligible for lifetime elision because it is a method (contains self) and so
        // the return type's lifetime is auto annotated to that of self (Step/Rule 3)
        fn multiple_inputs_with_self(&self, p1: &str, p2: &str, p3: &str) -> &str {
            println!("multiple_inputs_with_self takes in 3 params: {}, {}, {} and returns first page", p1, p2, p3);
            self.page_1 // returns first page
        }
        
        // The function bellow is partially eligible for lifetime elision, the difference between this and the
        // function above is that it sets the page field to that of p1, and for that to happen it needs to 
        // know the life time of p1 to compare it against that of page_1 and so we have to explicitly annotate
        // that particular param with the lifetime of the structure/impl.

        fn multiple_inputs_with_self_setter(&mut self, p1: &'a str, p2: &str, p3: &str) -> &str {
            println!("multiple_inputs_with_self takes in 3 params and sets, then returns first page: {}, {}, {}", p1, p2, p3);
            self.page_1 = p1;
            self.page_1 // returns first page
        }
    }

    let mut manual = Manual::default();

    manual.multiple_inputs_with_self("content_1", "content_2", "content_3");
    
    manual.multiple_inputs_with_self_setter("content_1", "content_2", "content_3");
    
}
