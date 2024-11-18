// //Rust’s standard library includes a number of very useful data structures called collections. 
// //Most other data types represent one specific value, but collections can contain multiple values. 
// //Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

// //Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. 


// //In this chapter, we’ll discuss three collections that are used very often in Rust programs:

// // 1. A vector allows you to store a variable number of values next to each other.
// // 2. A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
// // 3. A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.


// // To create a new empty vector, we call the Vec::new function, 
//  //let v: Vec<i32> = Vec::new();

// //To create a vector and then add elements to it, we can use the push method

// fn main() {
//     let mut v = Vec::new();
//     // 1. A vector allows you to store a variable number of values next to each other.
//     // 2. A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
//     // 3. A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

// v.push(5);
// v.push(6);
// v.push(7);
// v.push(8);

// //let ans = event_filter(v);

//     println!("{:?}", even_filter(&v));
//   // println!("{:?}", ans);
//    println!("{:?}", v);

// }

// //test: A write a vector function that returns a n even value in a vector 
// fn even_filter(vec: &Vec<i32>) -> Vec<i32>{
//     let mut new_vec = Vec::new();

//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(*val);
//         }
//     }
//     return new_vec;
// } 

// // fn add_even() -> Vec<i32> {
// //     let even_num = even_filter();
// //     let mut even_sum = even_num.iter().sum();

// //     return even_sum;
// // }

// vector can also be written like this

fn main(){
    let mut v = vec![5,6,7,8];
    println!("{:?}", v);

    v.push(9);
    v.push(10);
    v.push(11);

    println!("{:?}", v);

    // you can print the values based on their index 
    let third_value = &v[2];
    println!("The third value is: {}", third_value);
}


// HASHMAPS

