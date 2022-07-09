//Docs Ref (https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

 
//The following modules tree simply reflects the actual filesystem tree of files under instruments folder.
//It defines the mod tree in which the compiler need to look up to find the files, moreover they are preceeded
//by the 'pub' keyword to allow them to be accessible, by default they are private.
//This might look a little verbose, but one doesn't necessarily have to define all the tree in this file.
//nested submodules can be defined per relevant submodues, and only direct submodules/files can be listed here instead
//but we expose the whole tree to the compiler in this file to easily understand the structure.

pub mod instruments {

    pub mod instrument;

    pub mod string {
        pub mod cello;
        pub mod guitar;
        pub mod violin;
    }

    pub mod percussion {
        pub mod cajon;
        pub mod drums;
    }

    pub mod wind {
        pub mod brass {
            pub mod horn;
            pub mod trumpet;
        }

        pub mod wood {
            pub mod clarinet;
            pub mod flute;
        }
    }
}


//Use statememts are basically path shortcuts, every other nested submodule shall be accessed by "::"
//to reduce verbosity, we can use the 'use' statements in the following manner to be able to directly 
//access the module or their pub definitions by their final name.
//If you are familiar with C++, this vaguely resembles accessing namespaces.

//Modules paths can be accessed by either Absolute paths or Relative paths.

//This is an Absolute path starting from the crate root file (main.rs for bin or lib.rs for libs).
use crate::examples::modules::instruments::instrument::Instrument;

//This is a relative path from the current module's path.
use instruments::percussion::{cajon::Cajon, drums::Drums};
use instruments::string::{cello::Cello, guitar::Guitar, violin::Violin};
use instruments::wind::{
    brass::{horn::Horn, trumpet::Trumpet},
    wood::{clarinet::Clarinet, flute::Flute},
};

#[test]
pub fn main() {

    crate::example_prologue!("Modules Demo - Playing instruments");

    //We create an array of instruments and iterate over them and call the play function.
    //PS: Box type is a wrapper to give size to non sized things like traits, we will discuss it in a later chapter.
    let instruments: [Box<dyn Instrument>; 9] = [
        Box::new(Guitar::default()),
        Box::new(Cello::default()),
        Box::new(Violin::default()),
        Box::new(Drums::default()),
        Box::new(Cajon::default()),
        Box::new(Horn::default()),
        Box::new(Trumpet::default()),
        Box::new(Clarinet::default()),
        Box::new(Flute::default()),
    ];

    for inst in instruments {
        inst.play();
    }
}
