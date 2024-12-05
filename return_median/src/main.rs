//Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
//and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn main(){
    let mut arr = vec![2, 7, 1, 5, 4, 10];

    arr.sort();
    let len = arr.len();

   let mid = len / 2;

  let mid_value = if mid % 2 == 0 {
    let mid_average = (arr[mid - 1] + arr[mid]) as f64 / 2.0;

    mid_average
    
  }else {

    arr[mid] as f64    

  };


    println!("Midvalue is {:?}", mid_value);
}