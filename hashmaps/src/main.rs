// HashMap
// hashmaps stores key-value pairs. we it to be as object in javascript.
// Just like vectors, hash maps store their data on the heap

// methods: (1) insert: to insert something into the hashmap (2) remove: to remove something from the hashmap (3) get: to get something from the hashmap
// (4) clear: to clear the hashmap

// How do you define a hashmap
// (1) import it from the standard library  'use std::collections::HashMap'
use std::collections::HashMap;


// (2) use the new key word

fn main(){
   let mut users: HashMap<String, u32> = HashMap::new();
    // or - still the same as the above:
    // let mut users = HashMap::new(); i prefer this.

    // let mut users = HashMap::new();

    users.insert(String::from("Blue"), 10);
    users.insert(String::from("Red"), 50);

    // you can see the example above as we keeping track of the scores of two teams whose names are Blue and Yellow. 
    //The Blue team starts with 10 points, and the Yellow team starts with 50.


   // println!("{:?}", users);
    //getting something from the hashmap

    let team_one = users.get("Blue");
    let team_two = users.get("Red");

    match team_one {
        Some(score) => println!("Blue team has {} points", score),
        None => println!("Blue team not found"),
    }

    match team_two {
        Some(score) => println!("Red team has {} points", score),
        None => println!("Red team not found"),
    }

}


// we can iterate through each value and pair in the hashmap 
fn main(){
    let mut users: HashMap<String, u32> = HashMap::new();
    users.insert(String::from("Blue"), 10);
    users.insert(String::from("Red"), 50);
    users.insert(String::from("Green"), 100);

    for (team, score) in &users {
        println!("{} team has {} points", team, score);
    }
}



fn main() {
    let mut users: HashMap<String, i32> = HashMap::new();
    
    // Correct way to insert a key-value pair
    users.insert(String::from("Blue"), 10);

    println!("{:?}", users);
}


// Task: Write a code that takes in a vector of struct and return a hashmap

fn struct_to_hashmap(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }

    return hm;
}

fn main(){
    // a vector that takes in tuble. in our case, we take the tuble as our struct
    let vec_structs = vec![(String::from("Blue"), 20), (String::from("Red"), 50)];
    let hm = struct_to_hashmap(vec_structs);

    println!("{:?}", hm);
}