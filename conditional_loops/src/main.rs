// // if else in Rust
// fn main(){
//     let user_status: bool = false;

//     if user_status {
//         println!("Welcome to the World of Rust!");
//     } else if !user_status {
//         println!("Sorry, you are not allowed to enter this world.");
//     }

//     println!("Are you getting the flow?")
// }

// //looping
// fn main(){
//     // Note: we can iterate through arrays, maps and Strings
//     for i in 0..5 {
//          let x: i32 = i * 100;

//         println!("{}", x);
//     }


// }


//three different ways to loop throug an array

// 1. using a for loop
// fn main(){
//     let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

//     for x in numbers.iter(){
//         println!("{}", x);
//     }
// }

// 2. Using an index-based loop
// fn main(){
//     let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

//     //In Rust, the len() method is used to get the number of elements in a collection, 
//     //such as an array, vector, or string. 
//     //It returns a usize (an unsigned integer type) representing the count of elements.
//     for z in 0..numbers.len(){
//         println!("{}", numbers[z]);
//     }
// }

// 3. Using while loops
// fn main(){
//     let numbers = [1, 2, 3, 4, 5, 6, 7];

//     let mut index = 0;

//     while index < numbers.len(){
//         println!("{}", numbers[index]);
//         index += 1;
//     }
// }

// 4. Using for with enumerate
//If you need both the index and the value, you can use enumerate:
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for (index, &number) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, number);
    }
}


