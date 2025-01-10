use std::io;

fn main() {
    // Convert temperatures between Fahrenheit and Celsius.
    let mut user_input: String = String::new();
    let mut direction: String = String::new();
    println!("Enter a temperature to convert:");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: f64 = user_input.trim().parse().expect("Please type a number!");

    println!("Do you want to convert to or from Celsius? (t/f)");

    io::stdin()
        .read_line(&mut direction)
        .expect("Failed to read line");

    let mut direction = direction.trim().to_string();

    let possible = ["t", "f"];
    while !possible.contains(&direction.as_str()) {
        println!("Incorrect input, please try again");
        direction.clear();
        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");
    }

    let output: f64;

    if direction == "t" {
        println!("Converting {user_input:.2} degrees F to Celsius");
        output = (user_input - 32.0) * (5.0 / 9.0);
    } else {
        println!("Converting {user_input:.2} degree C to Fahrenheit");
        output = (user_input * (9.0 / 5.0)) + 32.0;
    }

    println!("Result: {output:.2}")
}
