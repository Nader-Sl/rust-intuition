//Book Ref: https://doc.rust-lang.org/book/ch10-02-traits.html

// A trait defines functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way. We can use trait bounds
// to specify that a generic type can be any type that has certain behavior.
//T raits in concept are similar to interfaces in other languages like Java but with slight differences.

// Keep in mind that Rust doesn't have the conventional 'hierarchal inheritance' concept.
// so traits are often the 'goto' when it comes to providing some shared functionality.

///// Lets come up with a game logic to better demonstrate traits and keep it fun. /////

// **Using derive attribute macro is essentially telling the compiler to implement the 'Debug'
// trait for us for our struct/enums.

#[derive(Debug)]
enum WoodTexture {
    Oak,
    Willow,
    Yew,
}

#[derive(Debug)]
enum ClothTexture {
    Tactical,
    Ninja,
    Unicorn,
}

#[derive(Debug)]
enum Texture {
    Wood(WoodTexture),
    Cloth(ClothTexture),
}

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Entity {
    location: Vector2,
    name: String,
    texture: Texture,
}

#[derive(Debug)]
struct Player {
    entity: Entity,
}

#[derive(Debug)]
struct NPC {
    entity: Entity,
}

#[derive(Debug)]
struct Door {
    entity: Entity,
}

#[derive(Debug)]
struct Chest {
    entity: Entity,
}
//Interaction trait to be implemented by both Door & Chest.
trait Interaction: std::fmt::Debug { // traits can extend a super trait or multiple using the '+' operator.

    // Traits can either contain function prototypes to be implemented for a particular type
    // or have a default impl and can be overriden by another more concrete implementation.

    //defaulted function
    fn open(&self) {
        println!("Called the default Interactable::open function");
    }

    //prototype function
    fn lock(&self);
}

impl Interaction for Door {
    //override default open
    fn open(&self) {
        println!("Called the Door::Open function");
    }

    fn lock(&self) {
        println!("Called the Door::lock function");
    }
}

impl Interaction for Chest {
    //Does not override the open function, default one would be called.

    fn lock(&self) {
        println!("Called the Chest::lock function");
    }
}

//Mobility trait to be implemented by both Player & NPC.
trait Mobility {
    type T; //T is a trait associated type. Its objective is to replace using generics on a trait when possible to improve code readabililty and maintainability.
    fn move_to(&self, _: &Self::T); // self refers to the implementing object, and Self refers to the type of the implementing object.
}

impl Mobility for Player {
    type T = Vector2; //We can specify the type of the associated type in here, we move a play to any position vector.
    fn move_to(&self, dest: &Vector2) {
        println!("Moving Player to destination => {:?}", dest);
    }
}

impl Mobility for NPC {
    type T = Box<dyn Interaction>; //Npcs can move to any interactable entity like a door or chest but not to a raw vector position.

    // We have chosen to use a Box wrapper in here (refer to smart_pointers.rs) because we can't use Interaction trait type directly
    // since its size is not known at compile time. Also since the underlying concrete type implementing the 'Interaction' trait is
    // unknown at compile time (could be door or chest), the 'dyn' keyword has to be prefixed to the trait name to facilitate the
    // resolution of the exact trait implementing type at run time via Virtual table referencing.

    fn move_to(&self, interactable: &Self::T) {
        println!("Moving NPC towards interactable => {:?}", interactable);
    }
}

#[test]
pub fn main() {
    crate::example_prologue!("Traits");

    //Let's create an instance of each entity type.

    let player = Player {
        // stack instance.
        entity: Entity {
            location: Vector2 { x: 0.0, y: 0.0 },
            name: "Player1".to_owned(),
            texture: Texture::Cloth(ClothTexture::Tactical),
        },
    };

    let npc = NPC {
        //stack instance.
        entity: Entity {
            location: Vector2 { x: 0.0, y: 0.0 },
            name: "NPC1".to_owned(),
            texture: Texture::Cloth(ClothTexture::Ninja),
        },
    };

    // We can return an object of a boxed dynamic Interaction type in here for the same reason
    // explined in the NPC implementation of the Mobility trait, therefore the interactables
    // are going to be placed on the heap.

    fn spawn_random_interactable(name: String) -> Box<dyn Interaction> {
        use rand::Rng; //using Rng from rand crate (https://docs.rs/rand/0.8.5/rand/trait.Rng.html)
        let mut rng = rand::thread_rng(); // random generator
        let rand_n = rng.gen_range(0..=10);

        let location = Vector2 {
            // randomly generate a location
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0.0..100.0),
        };

        let texture = if rand_n % 2 == 0 {
            //randomly generate a texture
            Texture::Wood(WoodTexture::Oak)
        } else {
            Texture::Wood(WoodTexture::Willow)
        };

        //Spawn a random interactable on the heap.
        match rand_n {
            0..=4 => Box::new(Door {
                entity: Entity {
                    location,
                    name,
                    texture,
                },
            }),
            _ => Box::new(Chest {
                entity: Entity {
                    location,
                    name,
                    texture,
                },
            }),
        }
    }

    let mut interactables = Vec::new(); // vector of interactable objects.

    // Spawn 5 interactables of random types (door or chest).
    for i in 0..5 {
        let interactable =
            spawn_random_interactable("Interactable_".to_owned() + i.to_string().as_str());
        interactables.push(interactable);
    }

    //Move player to a new location.
    player.move_to(&Vector2 { x: 2.0, y: 2.0 });

    // Move NPC to all spawned interactables.
    for i in &interactables {
        npc.move_to(&i);
    }

    ////// Traits as Parameters //////

    // Functions can accept a trait or a combination of traits from various types.
    // in this case we want to accept only (interactable or Mobile) objects but also we want them to
    // have the 'debug' trait as well via the #[derive(Debug)] so we can debug print it.
    // We can specifiy to only allow objects implementing those two traits by adding
    // those two traits via the '+' sign (for multiple traits).

    // All type parameters have an implicit bound of Sized (known at compile time) but we need to use a
    //special syntax '?Sized' to remove this bound if we need to pass in a dynamic trait object (known only at runtime)
    fn print_interactable(i: &(impl Interaction + std::fmt::Debug + ?Sized)) {
        println!("Printing Interactable: {:?}", i);
    }

    fn print_mobile(i: &(impl Mobility + std::fmt::Debug)) {
        println!("Printing Mobile: {:?}", i);
    }

    print_mobile(&player);
    print_mobile(&npc);

    for i in &interactables {
        print_interactable(i.as_ref()); //Get the reference from the Box smart pointer.
    }

    ////// Trait bound syntax //////
    ///
    //The impl Trait syntax we saw earlier works for straightforward cases but is actually syntax sugar
    //for a longer form known as a 'trait bound' which utilizes generics (check out ./generics.rs) it looks like this.

    fn print_mobile_classic<T: Mobility + std::fmt::Debug>(i: &T) {
        println!("Printing Interactable (classic): {:?}", i);
    }

    print_mobile_classic(&player);
}
