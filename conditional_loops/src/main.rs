// if else in Rust
fn main(){
    let user_status: bool = false;

    if user_status {
        println!("Welcome to the World of Rust!");
    } else if !user_status {
        println!("Sorry, you are not allowed to enter this world.");
    }

    println!("Are you getting the flow?")
}

//looping
fn main(){
    // Note: we can iterate through arrays, maps and Strings
    for i in 0..5 {
         let x: i32 = i * 100;

        println!("{}", x);
    }


}
