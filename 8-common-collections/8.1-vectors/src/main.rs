fn main() {
    let mut v: Vec<i32> = Vec::new();


    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element")
    }

    // v = vec![1, 2, 3, 4];

    for i in &v {
        println!("{i}")
    }

    let mut v1 = vec![100, 32, 57];

    for i in &mut v1 {
        *i += 50;
        println!("{i}");
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(25),
        SpreadsheetCell::Float(2.5),
        SpreadsheetCell::Text(String::from("Hello world")),
    ];




} // all vectors are dropped when out of scope