pub mod examples {
    pub mod common_concepts;
    pub mod macros;
    pub mod ownership;
    pub mod prototype_structures {
        pub mod enums;
        pub mod structs;
        pub mod unions;
    }

    pub mod collections;
    pub mod error_handling;
    pub mod modules;

    pub mod advanced {
        pub mod generics;
        pub mod traits;
    }
}

fn main() {
    println!("Examples are in the ./examples folder.\n");

    //Common Concepts
    examples::common_concepts::common_data_types();
    examples::common_concepts::vars_and_mut();
    examples::common_concepts::control_flow();
    examples::common_concepts::var_shadowing();
    examples::common_concepts::functions();

    //Ownership
    examples::ownership::assignment_ownership();
    examples::ownership::function_ownership();
    examples::ownership::refs_and_burrowing();
    examples::ownership::slice_type();
    examples::ownership::dangling_reference();

    //oop
    examples::prototype_structures::structs::struct_def_and_init();
    examples::prototype_structures::structs::structs_flavors();
    examples::prototype_structures::structs::structs_mutability();

    examples::prototype_structures::enums::enums_basic();
    examples::prototype_structures::enums::enums_advanced();

    examples::prototype_structures::unions::main();

    //modules
    examples::modules::main();

    //collections
    examples::collections::collection_vec();
    examples::collections::collection_hashmap();


    /////////////////////////// Advanced ///////////////////////////////////
    
    //traits
    examples::advanced::traits::main();
    //generics
    examples::advanced::generics::manual_duplication();
    examples::advanced::generics::generic_duplication();
    examples::advanced::generics::refs_lifetime_validation();

    //Error Handling -- Runs last since it demonstrates fatal errors => program termination.
    //Comment the following lines incrementally one at a time after testing each line to try them all out,
    //since each call can be fatal and terminate the application.

    examples::error_handling::Panic::intentional();
    examples::error_handling::Panic::invalid_memory();
    examples::error_handling::Result_Handling::panic_if_err_1();
    examples::error_handling::Result_Handling::panic_if_err_2();
    examples::error_handling::Result_Handling::panic_if_err_3();
    examples::error_handling::Result_Handling::match_err_type();
    examples::error_handling::Result_Handling::propagate_error();
    examples::error_handling::Result_Handling::custom_result();
}
