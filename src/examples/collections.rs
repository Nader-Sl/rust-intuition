//Doc Ref : https://doc.rust-lang.org/std/collections/index.html

/*
Rust’s collections can be grouped into four major categories:
    - Sequences: Vec, VecDeque, LinkedList
    - Maps: HashMap, BTreeMap
    - Sets: HashSet, BTreeSet
    - Misc: BinaryHeap
*/

// Examples will only be introducing the two most commonly used container types: Vec and HashMap.

pub fn collection_vec() {
    crate::example_prologue!("collection_vec");

    /*
       The first collection type we’ll look at is Vec<T>, also known as a vector.
       Vectors allow you to store more than one value in a single data structure that puts all
       the values next to each other in memory. Vectors can only store values of the same type.
       They are useful when you have a list of items, such as the lines of text in a file or
       the prices of items in a shopping cart.
    */

    //lets create an inventory of weapon names.
    //'inventory' should be mutable because we need to modify it later by pushing data to it.
    let mut inventory: Vec<String> = Vec::new();
    //push the elements into the array.
    inventory.push("AK47".to_owned());
    inventory.push("FAMAS".to_owned());
    inventory.push("P90".to_owned());
    inventory.push("SCAR".to_owned());

    //Alternatively we can declare a non-mutable owner in combination with the vec! macro which will automate what's done above.
    let inventory: Vec<String> = vec![
        "AK47".to_owned(),
        "FAMAS".to_owned(),
        "P90".to_owned(),
        "SCAR".to_owned(),
    ];

    //Iterate over the inventory container by reference to void moving vector elements into the loop so we can use it afterwards.
    for weapon in &inventory {
        println!("Iterating over weapon : {}", weapon);
    }
}

pub fn collection_hashmap() {
    crate::example_prologue!("collection_hashmap");

    /*
        Hash maps are useful when you want to look up data not by using an index.
        as you can with vectors, but by using a key that can be of any type.
        For example, in a game, you could keep track of each team’s score in a hash map
        in which each key is a team’s name and the values are each team’s score.
        Given a team name, you can retrieve its score.
    */

    //Let's create a map for storing weapons and their corresponding prices.

    //We first have to use that module from the std lib since its not not included
    //in the features brought into scope automatically in the prelude.
    use std::collections::HashMap;

    let mut weapons_db = HashMap::new(); //create a new hashmap

    //insert into map the key/pair, weapon name being the key, and the corresponding price being the value.
    weapons_db.insert("AK47".to_owned(), 3000);
    weapons_db.insert("FAMAS".to_owned(), 25000);
    weapons_db.insert("P90".to_owned(), 2350);
    weapons_db.insert("SCAR".to_owned(), 4000);

    //There's another cool way to construct a hashmap if we happened to have two
    //separate preconstructed vecs one representing the keys and the other the values.

    let inventory: Vec<String> = vec![
        "AK47".to_owned(),
        "FAMAS".to_owned(),
        "P90".to_owned(),
        "SCAR".to_owned(),
    ];

    let prices = vec![3000, 25000, 2350, 4000];

    //The following line of code uses the **zip** function which zips the values from both
    //iterators in parallel to form a new pair iterator. We finally call the **collect** function
    //to turn the iterator back into collection.
    let mut weapons_db: HashMap<_, _> = inventory.iter().zip(prices.iter()).collect();

    //iterate the map (key,val) by reference.
    println!("Iterating the weapons db");
    for (weapon, price) in &weapons_db {
        println!("The price of {} is {}", weapon, price);
    }

    //Update the price (value) of FAMAS, 2nd entry in weapons inventory.
    
    let weapon = &inventory[1]; // FAMAS weapon
    let new_price = 20000;
    let old_entry = weapons_db.insert(weapon, &new_price);
    //The 'insert' function returns the old value before replacement, it is wrapped by option since the value 
    //since there might not be an older entry with the same key in the map. in that case the option would
    //be of an enum value 'none' but since we can guarantee that the entry is already there, we can just call 
    //unwrap to access the original value (price).
    println!("Updated the old price of {} from {} to {}", weapon, old_entry.unwrap(), new_price);  

    //Only Inserting a Value If the Key Has No Value by using entry and .or_insert combination.
    weapons_db.entry(&inventory[0]).or_insert(&5000); //won't update the old value since key is already in map.

    //next pair will get added to the map since it doesn't contain that new_weapon key.
    let new_weapon = "Mk18".to_owned();
    weapons_db.entry(&new_weapon).or_insert(&2200);

    //re-iterate the map (key,val) by reference.
    println!("\nRe-iterating the weapons db");
    for (weapon, price) in &weapons_db {
        println!("The price of {} is {}", weapon, price);
    }
}
