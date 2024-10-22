fn main() {
    println!("Hello, world!");
    stack_fn();
    heap_fn();
    update_string()
}


fn stack_fn(){
    // this is stored on the stack memory
    let x: i32 = 90;
    let y: i32 = 12;

    let z: i32 = x + y;

    println!("The sum of {}, and {} is: {}", x, y, z);
}

fn heap_fn(){
    // this is stored on the heap memory
    let s1: String = String::from("Hello, World!");
    let s2: String = String::from(" You want to learn Rust?");
    let combine = format!("{}{}", s1, s2);

    println!("The combine string of heap_fn is: {}", combine);
}

fn update_string(){
    //note:you can't update the string without adding the mut expression
    
    let mut s: String = String::from("Yo! I'd love to learn Rust");
    println!("Before updating the string: {}", s);

    //updated string
    s.push_str(" but i find it difficult to find a good tutor. would you like to be my Rust tutor?");
    println!("After updating the string: {}", s);
}