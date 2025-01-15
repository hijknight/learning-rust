fn main() {
    let data = "initial";
    let mut s = data.to_string(); // returns String

    println!("{}", s);

    s.push_str(" contents"); // returns String, takes &str

    println!("{}", s);

    let mut s1: String = String::from("Hello");
    let s2: &str = " there";

    s1.push_str(s2);

    println!();
    println!("{}", s2);

    let mut s = String::from("lo");

    s.push('l'); //takes char and pushes to String s

    println!("{}", s);

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s4: &str = "there";
    // let s3: String = s1 + s4; // &str type

    let s3: String = s1 + &s2; // String reference is &String type,
    // but compiler can coerce &String into &str
    // Uses add method in String implementation

    println!("{}", s3);

    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = String::from("baz");

    let s = format!("{s1}, {s2}, {s3}"); // returns a String

    println!("{s}");


    let hello = String::from("Hola");
    let cyrillian = String::from("Здравствуйте");
    let h = &hello[..1];

    println!("{}", h);

    // iterating over String

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

}
