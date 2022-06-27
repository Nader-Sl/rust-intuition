//Doc Ref https://doc.rust-lang.org/stable/reference/items/unions.html

//A union declaration uses the same syntax as a struct declaration, except with union in place of struct.

// The key property of unions is that all fields of a union share common storage.
// As a result, writes to one field of a union can overwrite its other fields, and size
// of a union is determined by the size of its largest field.
// This is most useful when the type of data being passed through functions is unknown,
// using a union which contains all possible data types can remedy this problem.

pub fn main() {
    
    crate::example_prologue!("Unions");

    use std::mem;

    // Unions in Rust can have non-zero offsets and some other different guarantees than
    // a conventional C Enum (from the C lang), if we care to have it conform to a C enum
    // for interopability and what not, we can mark it with the #[repr(C)] attribute.
    // which guarantees it to have the same size and alignment as an equivalent C union declaration
    // in the C language  for the target platform. The union will have a size of the maximum size of
    // all of its fields rounded to its alignment, and an alignment of the maximum alignment of all
    // of its fields. These maximums may come from different fields.

    #[repr(C)] // C union conformation attribute.
    union CharOrInt {
        a: char, //Size = 1 byte
        b: i32,  //Size = 4 bytes
    } // Total size = size of largest field (b) = 4 bytes.

    //Declare an instance
    let mut u = CharOrInt { b: 32 };

    // Accessing and writing to a union in Rust is considered unsafe and must be
    // scoped within an 'unsafe' block. with the exception of writing to a Copy traited
    // type or a ManuallyDropped field.

    u.b = 33; //We don't need to wrap in unsafe scope since we are writing to a copyable field.

    // Although the unions fields implements the Copy trait, we are reading and not writing
    // therefore we still need to wrap it in an unsafe scope.
    unsafe {
        println!(
            "Created a union of type CharOrInt of size = {}, a = {}, b = {}",
            mem::size_of::<CharOrInt>(),
            u.a,
            u.b
        )
    }
}
