// fn main() {
//    // Structs are like objects in javascript

//    struct Person{
//     name: String,
//     age: u32,
//     city: String,
//    }

//    let name: String = String::from("Nathaniel Code");
//    let person: Person = Person{
//     name: name,
//     age: 25,
//     city: String::from("New York"),
//    };

//    println!("Hello, {}!, i was told you are {} year old and you live in {}. How true is this?", person.name, person.age, person.city);
// }

// implement structs
// You can implement structs, which means you can attach functions to instances 
// of structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}