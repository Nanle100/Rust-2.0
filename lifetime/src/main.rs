pub fn call() -> (String, usize) {
    let mut s = String::from("hello");
    s.push_str(", world");

    //let s2 = &s;
    let length = s.len();

    // println!("{}", s2); // uses an old reference that has been changed `s2`
    //  let length = s.len();

    (s, length)
}

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{}", s);

    let length = s.len();

    println!("Length of the string: {}", length);
}

struct User<'a> {
    name: &'a str,
}
fn main() {
    let name = String::from("Nanle");
    let user = User { name: &name };

    println!("{}", user.name);
}
