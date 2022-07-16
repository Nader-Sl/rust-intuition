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

trait EuclideanDist {
    fn distance(&self, other: &Self) -> f32;
}

impl EuclideanDist for Vector2 {
    fn distance(&self, other: &Self) -> f32 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }
}
//Interaction trait to be implemented by both Door & Chest.
trait Interaction : std::fmt::Debug {
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
    fn move_to(&self, _: &Self::T);// self refers to the implementing object, and Self refers to the type of the implementing object.
}

impl Mobility for Player {
    type T = Vector2; //We can specify the type of the associated type in here, we move a play to any position vector.
    fn move_to(&self, dest: &Vector2) {
        println!(
            "Moving Player to destination => {:?}",
            dest
        );
    }
}

impl Mobility for NPC {
    type T = Box<dyn Interaction>; //Npc can move to any interactable entity like a door or chest but not to a raw vector position.
  
    // We have chosen to use a Box wrapper in here (refer to smart_pointers.rs) because we can't use Interaction trait type directly
    // since its size is not known at compile time. Also since the underlying concrete type implementing the 'Interaction' trait is 
    // unknown at compile time (could be door or chest), the 'dyn' keyword has to be prefixed to the trait name to facilitate the 
    // resolution of the exact trait implementing type at run time via Virtual table referencing. 

    fn move_to(&self, interactable: &Self::T) {
        println!(
            "Moving NPC towards interactable => {:?}",
            interactable
        );
    }
}

#[test]
pub fn main() {
    crate::example_prologue!("Traits");

    //Let's create an instance of each entity type.

    let player = Player {
        entity: Entity {
            location: Vector2 { x: 0.0, y: 0.0 },
            name: "Player1".to_owned(),
            texture: Texture::Cloth(ClothTexture::Tactical),
        },
    };

    let npc = NPC {
        entity: Entity {
            location: Vector2 { x: 0.0, y: 0.0 },
            name: "NPC1".to_owned(),
            texture: Texture::Cloth(ClothTexture::Ninja),
        },
    };

    let door = Door {
        entity: Entity {
            location: Vector2 { x: 0.0, y: 0.0 },
            name: "door1".to_owned(),
            texture: Texture::Wood(WoodTexture::Willow),
        },
    };

    let chest: Box<dyn Interaction + 'static> = Box::new(Chest {
        entity: Entity {
            location: Vector2 { x: 0.0, y: 0.0 },
            name: "chest1".to_owned(),
            texture: Texture::Wood(WoodTexture::Oak),
        },
    }) ;

    //Move player to a new location.
    
    player.move_to(&Vector2 { x: 2.0, y: 2.0 });

    // Move NPC to a chest.
 
    npc.move_to(&chest);
    ////// Traits as Parameters //////

    // Functions can accept a trait or a combination of traits from various types.
    // in this case we want to accept only (interactable or Mobile) objects but also we want them to
    // have the 'debug' trait as well via the #[derive(Debug)] so we can debug print it.
    // We can specifiy to only allow objects implementing those two traits by adding
    // those two traits via the '+' sign (for multiple traits).

    // All type parameters have an implicit bound of Sized (known at compile time) but we need to use a 
    //special syntax '?Sized' to remove this bound if we need to pass in a dynamic trait object (known only at runtime)
    fn print_interactables(i: &(impl Interaction + std::fmt::Debug + ?Sized)) {  
        println!("Printing Interactable: {:?}", i);
    }

    fn print_mobile(i: &(impl Mobility + std::fmt::Debug)) {
        println!("Printing Mobile: {:?}", i);
    }

    print_mobile(&player);
    print_mobile(&npc);

    print_interactables(&door);
    print_interactables(chest.as_ref());

    ////// Trait bound syntax //////
    ///
    //The impl Trait syntax we saw earlier works for straightforward cases but is actually syntax sugar
    //for a longer form known as a 'trait bound' which utilizes generics (check out ./generics.rs) it looks like this.

    fn print_mobile_classic<T: Mobility + std::fmt::Debug>(i: &T) {
        println!("Printing Interactable (classic): {:?}", i);
    }

    print_mobile_classic(&player);
}
