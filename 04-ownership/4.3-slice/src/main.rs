// fn main() {
//     let s = String::from("hello world"); // first word ends at 5, so should return 5
//
//     let word: &str = first_word(&s);
//
//     println!("{word}");
// }

fn main() {
    let string = String::from("hello world");

    let word = first_word(&string[6..11]);
    let word = first_word(&string[..]);

    let word = first_word(&string);

    let str_lit = "Hello world";

    let word = first_word(&str_lit[0..5]);
    let word = first_word(&str_lit[..]);

    let word = first_word(str_lit);


    println!("{word}");

    let a = [1, 2, 3, 4, 5];

    let slice_a = &a[1..3];

    assert_eq!(slice_a, &[2, 3]);



}


// first word 1
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}



