use std::collections::HashMap;


pub fn mode(list: &[i32]) -> Option<i32> {

    if list.is_empty() {
        return None
    }
    let mut map = HashMap::new();

    for &num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num) // returns key with the highest count




}

pub fn median(mut list: Vec<i32>) -> Option<f64> {

    if list.is_empty() {
        return None;
    }

    list.sort();
    let len = list.len();
    if list.len() % 2 == 0 {

        Some((list[len / 2] as f64 + list[len / 2 - 1] as f64) / 2.0)

    } else {
        let mid = list.len() / 2;
        Some(list[len / 2] as f64)
    }

}