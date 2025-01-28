// Given a list of integers, use a vector and
// return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

mod lib;
use lib::{ median, mode };


fn main() {

    let nums = vec![1, 5, 5, 3, 3, 3, 6, 4, 2, 20, 10];


    match median(nums.clone()) {
        Some(m) => println!("Median: {}", m),
        None => println!("No median found"),
    }

    match mode(&nums) {
        Some(m) => println!("Mode: {}", m),
        None => println!("No mode found"),
    }
}









