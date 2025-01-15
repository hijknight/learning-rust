// Given a list of integers, use a vector and
// return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {

    let nums = vec![1, 5, 5, 3, 3, 3, 6, 4, 2, 20, 10];

    println!("median is {}\nmode is {}", median(nums.clone()), mode(nums));
}

fn median(mut list: Vec<i32>) -> f64 {
    list.sort();

    if list.len() % 2 == 0 {
        let mid = list.len() / 2;
        let mid_1 = &mid - 1;

        (list[mid] as f64 + list[mid_1] as f64) / 2.0

    } else {
        let mid = list.len() / 2;
        list[mid] as f64
    }

}

fn mode(list: Vec<i32>) -> i32 {

    let mut map = HashMap::new();

    for num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut mode = 0;

    for (key, value) in map {
        if value > max {
            max = value;
            mode = key;
        }
    }
    mode

}





