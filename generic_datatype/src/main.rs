// Generic Data Types

// We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types. 
//Let’s first look at how to define functions, structs, enums, and methods using generics. Then we’ll discuss how generics affect code performance.

// 1. In Function Definitions

// // - shows two functions that both find the largest value in a slice. We’ll then combine these into a single function that uses generics.
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'z', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {result}");
// }
// so basically this is how our functions were defined: 'fn largest<T>(list: &[T]) -> &T {}'



// 2. In Struct Definitions

// We can also define structs to use a generic type parameter in one or more fields using the <> syntax. 
// defines a Point<T> struct to hold x and y coordinate values of any type.
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// we define our struct to be generic in <T> this means that we are expecting the value of x and y to be the same. it can either be an inter or a floating number as seen above

// but look at this code: 
// what do you think?
// struct Point<T> {
//     x: T,
//     y: T,
// }
// fn main() {
//     let wont_work = Point { x: 5, y: 4.0 };
// }
// this will return an error as our struct gave it variables a common data type T, each varialbe, x and y can not be different from the other
// so we have x as an integer and y as a floating number. this will return an error. the variables can't have different datatypes; they can onlt have one, either 
// an integer or a folating number



// hnece to make our struct accept variable of different datatype we must let it know from it generics, like this:
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// the value of x and y are clearly define to be different from their generics
// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }


// 3. In Enum Definitions

// As we did with structs, we can define enums to hold generic data types in their variants. 
// Let’s take another look at the Option<T> enum that the standard library provides
// enum Option<T> {
//     Some(T),
//     None,
// }

// This definition should now make more sense to you. As you can see, the Option<T> enum is generic over type T and has two variants: 
// Some, which holds one value of type T, and a None variant that doesn’t hold any value. 
// By using the Option<T> enum, we can express the abstract concept of an optional value, and because Option<T> is generic, we can use this abstraction no matter what the type of the optional value is.

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
//The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a value of type T, and Err, which holds a value of type E. 
//This definition makes it convenient to use the Result enum anywhere we have an operation that might succeed (return a value of some type T) or fail (return an error of some type E). 
//In fact, this is what we used to open a file in Listing 9-3, where T was filled in with the type std::fs::File when the file was opened successfully and E was filled in with the type std::io::Error when there were problems opening the file.



// 4. In Method Definitions
// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition. 
// Here, the generic parameters X1 and Y1 are declared after impl because they go with the struct definition. 
// The generic parameters X2 and Y2 are declared after fn mixup because they’re only relevant to the method.

