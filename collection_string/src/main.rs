//Creating a new string

fn main(){
// This line creates a new, empty string called s, into which we can then load data
    let mut s = String::new();

    //here is how we load data
    let data = "initial contents";

    let s = data.to_string();


    // note: We can also use the function String::from to create a String from a string literal
    let s = String::from("initial contents");


    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

}

//Updating a String

//A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it. 
//In addition, you can conveniently use the + operator or the format! macro to concatenate String values.

//Appending to a String with push_str and push
//We can grow a String by using the push_str method to append a string slice,
// let mut s = String::from("foo");
// s.push_str("bar");

fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    //Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used


}



fn main(){
    let s1 = String::from("hello");

    // returns the first strings character which is 'h'
    let h = &s1[0..1];

    println!("The first character is {}", h);
}