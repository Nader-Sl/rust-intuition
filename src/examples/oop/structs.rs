/*  A struct, or structure, is a custom data type that lets you package together
    and name multiple related values that make up a meaningful group.

    A struct definition only contains fields, adding functionality to it (methods)
    to it should be added to its implementation (impl) separately.
*/

//defining and instantiating structs
pub fn struct_def_and_init() {

    crate::example_prologue!("struct_def_and_init");

    //Defining a Rectangle struct.
    struct Rectangle {
        width: u32,
        height: u32,
    }

    //impl of methods for Rectangle struct.
    impl Rectangle {
        //reference to self as a function param points to the current instance of which this method is called upon.
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // A struct can define multiple impl blocks, but prefer to use one if in same file/location.
    impl Rectangle {
        // This is a static function that is listed under this struct, it doesn't require an instance of this struct
        // which explains why it lacks the &self param, it is invoked via Rectangle::default().
        fn default() -> Rectangle {
            Rectangle {
                width: 800,
                height: 600,
            }
        }
    }

    //Instantiating Structs
    let rect1 = Rectangle {
        width: 500,
        height: 500,
    };

    let area = rect1.area(); //invoke the instance method to get the area.
    println!(
        "The area of rect1 ({},{}) is {}.",
        rect1.width, rect1.height, area
    );

    let default_rect = Rectangle::default(); //Invoke the static function to create a new defaulted 'Rectangle' instance.
    println!(
        "The area of default_rect ({},{}) is {}.",
        rect1.width,
        rect1.height,
        default_rect.area()
    );
}

pub fn structs_flavors() {

    crate::example_prologue!("structs_flavors");

    // There are 3 main flavours to Rust structs.
    //  1 - standard struct with named fields.
    //  2 - Tuple structs: are like structs but with unnamed fields, they are used as tuples.
    //  3 - Unit-like structs: are structs without any fields, such structs can be useful when you need to
    //      implement a trait on some type but don’t have any data that you want to store in the type itself.

    //Standard struct
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    //Tuple struct
    #[derive(Debug)]
    struct Color(u32, u32, u32);

    //Unit-like struct
    #[derive(Debug)]
    struct UnitLike; //without paranthesis

    let rect = Rectangle{width:800, height: 600};
    let color = Color(255,255,255);
    let unit_like = UnitLike;

    println!("rect: {:?} => Standard struct", rect);
    println!("color: {:?} => Tuple struct", color);
    println!("unit_like: {:?} => UnitLike struct", unit_like);

}

pub fn structs_mutability() {

    crate::example_prologue!("structs_mutability");

    // By default structs are immmutable, therefore attempting to set one of its fields will result in a compilation error.
    // Structs' fields can't be set as mut (inherited mutability) otherwise the compiler would complain.
    // There are two common ways to mutate a struct's fields:
    //  1- Mark the struct owner as 'mut' when instantiating it.
    //  2- Allow interior mutability of the fields by using Cell or RefCell<T> for non threadsafe operations,
    //     otherwise use  Mutex<T>, RwLock<T> or atomic types for thread-safe operaitons.

    #[derive(Debug)] // The `derive` attribute automatically creates the implementation
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
    let mut user2 = User { ..user1 }; //This is much like ES6's spread operator but with two dots instead of 3.
    user2.active = true; //Compiles ok.
    println!("User2 (mut as a whole struct)= {:?}", user2);

    // now consider we only want to make one particular fields of the 'User' struct mutable.
    // for that we have to redefine the 'User' Struct and use one of the wrappers mentioned earlier.
    // we will choose to use the Cell wrapper for our purpose because we need to just edit the value
    // and we don't care about thread-safety because we are not using multiple threads.

    use std::cell::Cell;

    #[derive(Debug)] // The `derive` attribute automatically creates the implementation
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

    exUser.active.set(false); //We managed to do what we wanted, it compiles!
    println!(
        "exUser (with interiorly mutable field 'active' set to false ) = {:?}",
        exUser
    );
}
