enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

//Rust has an extremely powerful control flow construct called match that allows
// you to compare a value against a series of patterns and then execute code based on which pattern matches.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);



fn main(){

    let dice_roll = 9;
    match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

}


//Concise control flow with if let

//The if let syntax lets you combine if and let into a less verbose way to handle values
// that match one pattern while ignoring the rest.

//you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}


