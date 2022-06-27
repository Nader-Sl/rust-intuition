//Book Ref: https://doc.rust-lang.org/book/ch10-02-traits.html

// A trait defines functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way. We can use trait bounds
// to specify that a generic type can be any type that has certain behavior.
//Traits in concept are similar to interfaces in other languages like Java but with slight differences.

//Keep in mind that Rust doesn't have the conventional 'hierarchal inheritance' concept.
//so traits are often the 'goto' when it comes to providing some shared functionality.

///// Lets come up with a game logic to better demonstrate traits and keep it fun. /////

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

#[derive(Debug)]
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
    pub entity: Entity,
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

//Mobile trait to be implemented by both Player & NPC.
trait Mobile {
    // Traits can either contain function prototypes to be implemented for a particular type
    // or have a default impl and can be overriden by another more concrete implementation.
    fn move_by(&self, diff_vec: Option<Vector2>) {
        // If vector2 is not assigned, it will move by (1,1)
        println!(
            "Called the default Mobile::move_by with diff_vec = {:?}",
            diff_vec.unwrap_or(Vector2 { x: 1.0, y: 1.0 })
        );
    }
}

//Interactable trait to be implemented by both Door & Chest.
trait Interactable {
    // Traits can either contain function prototypes to be implemented for a particular type
    // or have a default impl and can be overriden by another more concrete implementation.

    //defaulted function
    fn open(&self) {
        println!("Called the default Interactable::open function");
    }

    //prototype function
    fn lock(&self);
}

impl Mobile for Player {
    fn move_by(&self, diff_vec: Option<Vector2>) {
        // If vector2 is not assigned, it will move by (2,2)
        println!(
            "Called the default Player::move_by with diff_vec = {:?}",
            diff_vec.unwrap_or(Vector2 { x: 2.0, y: 2.0 })
        );
    }
}

impl Mobile for NPC {
    fn move_by(&self, diff_vec: Option<Vector2>) {
        // If vector2 is not assigned, it will move by (3,3)
        println!(
            "Called the default Player::move_by with diff_vec = {:?}",
            diff_vec.unwrap_or(Vector2 { x: 3.0, y: 3.0 })
        );
    }
}

impl Interactable for Door {
    //override default open
    fn open(&self) {
        println!("Called the Door::Open function");
    }

    fn lock(&self) {
        println!("Called the Door::lock function");
    }
}

impl Interactable for Chest {
    //Does not override the open function, default one would be called.

    fn lock(&self) {
        println!("Called the Chest::lock function");
    }
}

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

    let chest = Chest {
        entity: Entity {
            location: Vector2 { x: 0.0, y: 0.0 },
            name: "chest1".to_owned(),
            texture: Texture::Wood(WoodTexture::Oak),
        },
    };

    ////// Traits as Parameters //////

    //Functions can accept a trait or a combination of traits from various types.
    //in this case we want to accept only (interactables or Mobile) objects but also we want them to
    //have the 'debug' trait as well via the #[derive(Debug)] so we can debug print it.
    //We can specifiy to only allow objects implementing those two traits by adding
    //those two traits via the '+' sign.

    fn print_interactables(i: &(impl Interactable + std::fmt::Debug)) {
        println!("Printing Interactable: {:?}", i);
    }

    fn print_mobile(i: &(impl Mobile + std::fmt::Debug)) {
        println!("Printing Mobile: {:?}", i);
    }

    print_mobile(&player);
    print_mobile(&npc);

    print_interactables(&door);
    print_interactables(&chest);

    ////// Trait bound syntax //////
    /// 
    //The impl Trait syntax we saw earlier works for straightforward cases but is actually syntax sugar  
    //for a longer form known as a 'trait bound' which utilizes generics (check out ./generics.rs) it looks like this.

    fn print_mobile_classic<T: Mobile + std::fmt::Debug>(i : &T){
        println!("Printing Interactable (classic): {:?}", i);
    }
  
    print_mobile_classic(&player);
}
