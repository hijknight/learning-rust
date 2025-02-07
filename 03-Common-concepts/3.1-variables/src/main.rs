use std::io;

fn main() {
    let a: [u32; 5] = [1, 2, 3, 4, 5];

    println! ("Please enter an array index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please type a number!");

    let element = a[index];

    println!("The element at index {} is {}", index, element);


}
