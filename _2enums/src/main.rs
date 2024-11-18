/**
* what is Enums?
* 1. Enumerations, also referred to as enums allow you to define a type by enumerating its possible variants. 

* contents to cover in this session:
* 1. Defining and using enums to show how an enum can encode meaning along with data.
* 2. we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing.
* 3. Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum.
* 4. we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.
 */

 //Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, 
 //enums give you a way of saying a value is one of a possible set of values. 
 //For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle. 
 //To do this, Rust allows us to encode these possibilities as an enum.


 // option enum
//  enum Option<T> {
//     None,
//     Some(T),
// }
 
// // The <T> syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter,
// //For now, all you need to know is that <T> means that the Some variant of the Option enum can hold one piece of data of any type, 
// //and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.

// //Here are some examples of using Option values to hold number types and string types:
// let some_number = Some(5);
// let some_char = Some('e');

// let absent_number: Option<i32> = None;


fn main(){
// we will need to keep doing this over and over again for any array we desire to add up. wahala!
    let array_one: [i32; 5] = [1, 2, 3, 4, 5];

    let array_two: [i32; 5] = [6, 7, 8, 9, 10];

    // let mut sum_arrayone:i32 = array_one[0];
    // let mut sum_arraytwo:i32 = array_two[0];
    
    // for i in 1..array_one.len(){
    //     sum_arrayone += array_one[i];
    // }

    // for i in 1..array_two.len(){
    //     sum_arraytwo += array_two[i];
    // }

    // println!("{}", sum_arrayone);
    // println!("{}", sum_arraytwo);

    // this how it should be with a function
     
    println!("{}", add_array(&array_one));
    println!("{}", add_array(&array_two));

}

// A better way of doing this is to create a fuction and does this adding up of our arrays

fn add_array(num: &[i32; 5]) -> i32 {
    let mut sum_array = num[0];

    for _i in 1..num.len(){
        sum_array += num[_i];
    }

    sum_array
}