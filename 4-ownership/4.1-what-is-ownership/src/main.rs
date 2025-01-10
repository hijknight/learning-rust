fn main() {
    // let mut s = String::from("hello");
    // s = String::from("ahoy");
    //
    // println!("{}, world!", s);

    let s = String::from("hello");


    {
        take_ownership(s);
    }


}
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}