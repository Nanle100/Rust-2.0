fn main() {
   let xm = do_sum(20, 10);
   println!("The sum value of xm is {}", xm);

   let pmz = do_divide(20, 10);
   println!("The quotient value of pmz is {}", pmz);

   let mm = my_formular(20, 10);
   println!("The result of my_formular is {}", mm);
}


fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn do_divide(k: i32, m: i32) -> i32 {
    return k / m;
}

fn my_formular(p: usize, q: usize) -> usize {
    return p * q / 20;
}    


