/*  A struct, or structure, is a custom data type that lets you package together
    and name multiple related values that make up a meaningful group.

    A struct definition only contains fields, adding functionality to it (methods)
    to it should be added to its implementation (impl) separately.
*/

//defining and instantiating structs
fn struct_def_and_init() {

    //Defining a Rectangle struct.
    struct Rectangle {
        width: u32,
        height: u32,
    }

    //impl of methods for Rectangle struct.
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    //Instantiating Structs
    let rect1 = Rectangle {
        width: 800,
        height: 600,
    };

    println!("The area of rectangle ({},{}) is {}.", rect1.width, rect1.height, rect1.area());
}

fn structs_flavors() {
    // There are 3 main flavours to Rust structs.
    //  1 - standard struct with named fields.
    //  2 - Tuple structs: are like structs but with unnamed fields, they are used as tuples.
    //  3 - Unit-like structs: are structs without any fields, such structs can be useful when you need to 
    //      implement a trait on some type but don’t have any data that you want to store in the type itself.

    //standard struct
    struct Rectangle {
        width: u32,
        height: u32
    }

    //Tuple struct
    struct Color (u32, u32, u32);

    //Unit-like struct
    struct UnitLike; //without paranthesis
}

fn structs_mutability() {

    // By default structs are immmutable, therefore attempting to set one of its fields will result in a compilation error.
    // Structs' fields can't be set as mut (inherited mutability) otherwise the compiler would complain.
    // There are two common ways to mutate a struct's fields:
    //  1- Mark the struct owner as 'mut' when instantiating it.
    //  2- Allow interior mutability of the fields by using Cell or RefCell<T> for non threadsafe operations,
    //     otherwise use  Mutex<T>, RwLock<T> or atomic types for thread-safe operaitons.

    #[derive(Debug)]  // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Uncommenting the following line will lead to a compilation error because structs fields are immutable by default.
    // user1.active = false;

    //Now lets create another inheritently mutable instance of 'User' based on the user1 instance above.
    let mut user2 = User { ..user1}; //This is much like ES6's spread operator but with two dots instead of 3.
    user2.active = true; //Compiles ok.
    println!("User2 (mut as a whole struct)= {:?}", user2);

    // now consider we only want to make one particular fields of the 'User' struct mutable.
    // for that we have to redefine the 'User' Struct and use one of the wrappers mentioned earlier.
    // we will choose to use the Cell wrapper for our purpose because we need to just edit the value
    // and we don't care about thread-safety because we are not using multiple threads.

    use std::cell::{Cell};

    #[derive(Debug)]  // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    struct ExUser {
        active: Cell<bool>,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let exUser = ExUser {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: Cell::new(true),
        sign_in_count: 1,
    };

    exUser.active.set(false);//We managed to do what we wanted, it compiles!
    println!("exUser (with interiorly mutable field 'active' set to false ) = {:?}", exUser);


}

fn main() {
    struct_def_and_init();
    structs_flavors();
    structs_mutability();
}
