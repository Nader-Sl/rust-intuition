
//Modules tree.
pub mod util;
#[cfg(test)]
pub mod examples {
    pub mod common_concepts;
    pub mod control_flow;

    pub mod macros;
    pub mod ownership;
    pub mod prototype_structures {
        pub mod enums;
        pub mod structs;
        pub mod unions;
    }

    pub mod collections;

    pub mod modules;

    pub mod advanced {
        pub mod generics;
        pub mod lifetime_specification;
        pub mod traits;

        pub mod smart_pointers;
        pub mod functional {
            pub mod closures;
            pub mod iterators;
        }

        pub mod unsafe_ops;
    }

    pub mod error_handling;
}

fn main() {}
