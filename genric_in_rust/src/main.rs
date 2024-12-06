// 1. Removing Duplication by Extracting a Function

//Before diving into generics syntax, let’s first look at how to remove duplication in a way that 
//doesn’t involve generic types by extracting a function that replaces specific values with a placeholder that represents multiple values. 
//Then we’ll apply the same technique to extract a generic function! By looking at how to recognize duplicated code you can extract into a function, you’ll start to recognize duplicated code that can use generics.

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

// now imagine we doing with:
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

// the code above is prone to errors. here is the best way of doing this:

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
   println!("Largest number in this array is: {result}");


    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("Largest number in this array is: {result}")
}

// In summary, here are the steps we took to change the code from Listing 10-2 to Listing 10-3:

// 1. Identify duplicate code.
// 2. Extract the duplicate code into the body of the function, and specify the inputs and return values of that code in the function signature.
// 3. Update the two instances of duplicated code to call the function instead.
