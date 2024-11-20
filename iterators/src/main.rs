// The iterator pattern allows you to perform some task on a sequence of items in turn

// (1) iterating using loops
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers {
        println!("Number: {}", num);
    }
}



// (2) Iterating after creating an 'iterator'
fn main (){
    let nums = vec![1, 2, 3, 4, 5];

    let iter = nums.iter();

    for num in iter {
        println!("Number: {}", num);
    }

    // why in the world would i need to use this method? Why do i need to have an ietrate variable when my loop can do the job?
}

// (3) itermut: if you want to mutate the iterator
fn main(){
    let mut nums = vec![1, 2, 3, 4, 5];

    let iter = nums.iter_mut();

    for value in iter {
        *value = *value * 2;

    }
    println!("{:?}", nums);
}


// (4) ternext

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    let mut iter = numbers.iter_mut();

    while let Some(num) = iter.next() {
        println!("Number: {}", num);
    }
}

//(5) intolter
it is use to to convert a collecttion into an iterator that takes ownership of the collection.
fn main(){
    let nums = vec![1, 2, 3, 4, 5];

    let iter = nums.into_iter();

    for value in iter {
        println!("Number: {}", value);
    }
}


// consuming adaptors : sum()

fn main () {
    let num = vec![1, 2, 3, 4, 5];

    let num_iter = num.iter();

   let sum_num: i32 = num_iter.sum();

   println!("Sum of numbers: {}", sum_num);
   // note: the sum function consumes the iterator and returns the sum of all elements. hence, we can no longer user the num_iter anymore

   let sum_num2: i32 = num_iter.sum();
   println!("Sum of numbers: {}", sum_num2); // this will give an error as the iterator is already consumed.

   

    println!("{:?}", num);
}


// we have another concept call ITERATOR ADAPTORS
// they are methods define on the iterator trait that don't consume the iterator. Instead, they produce different iterator by changing some aspect of the original iterator.

// 1. MAP
fn main(){
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];

    let iter1 = v1.iter();

    // map takes the iterated values from iter1 aas an arguement 'x' and multiply with 2 . map will eventually return a new iterator stored in new_iter
    // So you'd notice that map() doesn't consume the iterator; instead it does something with the iterated values and returns a new iterator(that is a multiplication by 2 of each iterated value).

    let new_iter = iter1.map(|x| x * 2);

    println!("new_iter: {:?}", new_iter);

    for value in new_iter {
        println!("Number: {}", value);
    }

    println!("{:?}", v1); // original vector remains the same

}


// 2. Filter
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let iter1 = v1.iter();

// this method creates a condition and return a new iterators that fits into the condition.
// in our code below, filter returns a new iteration of values that are even numbers.
    let new_iter = iter1.filter(|x| *x % 2 == 0);

    //println!("new_iter: {:?}", new_iter);

    for value in new_iter {
        println!("Number: {}", value);
    }

    println!("{:?}", v1); // original vector remains the same
}


// difference between map and filter
// Map does something with the iterations and return a new iterators while filter sets a condition and any iterated value to meets the condition is return as a new iterator.



// TODO: Write the logic to first filter all odd values then double each value and create a new vector
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Filter odd values
    let odd_values = v1.iter().filter(|x| **x % 2 != 0);
   
    // Double each odd value
    for value in odd_values {
        let doubled_value = *value * 2;
        println!("Number: {}", doubled_value);
    }

}
