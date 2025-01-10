fn main() {
    let mut s = String::from("hello world"); // first word ends at 5, so should return 5

    let word: usize = first_word(&s);

    s.clear();

    println!("{word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn second_word() {

}
