// fn main() {
//     let mut s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{s1}' is {len}.");
// }
//
// fn calculate_length(s: &String) -> usize {
//     return s.len();
// }

// change function
// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
//
//     println!("{}", s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


fn main() {
    let reference = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

// Let’s recap what we’ve discussed about references:
//
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
// Next, we’ll look at a different kind of reference: slices.