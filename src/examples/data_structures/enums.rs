
pub fn enums_basic() {

    crate::example_prologue!("enums_basic");
    
    //A simple enum that represents different message commands (like in most other programming languages)
    enum Message {
        Quit,
        Write,
        ChangeColor,
        Move
    }

    let command = Message::Move;

    //We can enum pattern match and only handle the "Quit" message.
    match command {
        Message::Quit => println!("Message : Quit, Terminating ..."),
        _ => () // rest ignored  
    }

}
pub fn enums_advanced() {
    
    crate::example_prologue!("enums_advanced");

    //Enums, besides the simple type, can have different variants that can associate data to them.
    enum Message {
        Quit, //simple type, no data associated.
        Write(String), // enum with associated unnamed data (Tuple variant)
        ChangeColor(i32, i32, i32),// enum with associated unnamed data (Tuple variant)
        Move { x: i32, y: i32 }, // enum with associated named data (Struct variant)
 
    }

    impl Message {
        fn call(&self) {
            match self{
                Message::Quit => println!("Msg : Quit"),
                Message::Write(str) => println!("Msg : Write {}", str),
                Message::Move{x, y} => println!("Msg : Move : ({},{})",x , y),
                Message::ChangeColor(r, g, b) => println!("Msg : ChangeColor : {},{},{}", r, g, b),
                _ => {},
            }
           
        }
    }

    let msg_queue = [
        Message::Write(String::from("Hello World")),
        Message::Move{x: 100, y:200},
        Message::ChangeColor(255,255,255),
        Message::Quit
    ];

    //Iterate the msg queue and call each's call function.
    for msg in msg_queue {
        msg.call();
    }
}
 