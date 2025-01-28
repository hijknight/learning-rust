use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Red"), 10);

    //let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");



    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    println!("{map:#?}");

    map.entry(String::from("Yellow")).or_insert(String::from("255, 255, 0"));
    map.entry(String::from("Favorite Color")).or_insert(String::from("Green"));
    println!("{map:#?}");

    println!();

    let text = "hello world wonderful world";

    let mut hash = HashMap::new();

    for word in text.split_whitespace() {
        let count = hash.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{hash:?}");
}
