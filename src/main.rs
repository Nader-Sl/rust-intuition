pub mod examples {
    pub mod common_concepts;
    pub mod macros;
    pub mod ownership;
    pub mod structs_and_enums;

    pub mod modules;
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

    //Structs and Enums.
    examples::structs_and_enums::struct_def_and_init();
    examples::structs_and_enums::structs_flavors();
    examples::structs_and_enums::structs_mutability();
    examples::structs_and_enums::enums();

    //modules
    examples::modules::main();

}
