/*  A struct, or structure, is a custom data type that lets you package together
    and name multiple related values that make up a meaningful group.

    A struct definition only contains fields, adding functionality to it (methods)
    to it should be added to its implementation (impl) separately.
*/

//defining and instantiating structs
 
use crate::*; //Import the entire crate.

pub fn struct_def_and_init() {

    example_prologue!("struct_def_and_init");

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

#[test]
pub fn structs_flavors() {

    example_prologue!("structs_flavors");

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
