use std::io;
fn main() {
    let user_input: f64 = 'input: loop {
        println!("Enter a temperature to convert in Celsius or Fahrenheit:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(value) => break 'input value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    };

    let direction: String = 'direction: loop {
        println!("Do you want to convert to or from celsius (t/f):");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed: String = input.trim().to_lowercase();

        if trimmed == "t" || trimmed == "f" {
            break 'direction trimmed.to_string();
        } else {
            println!("Invalid input. Please enter 't' of 'f'.");
        }
    };

    let output: f64;
    if direction == "t" {
        println!("Converting {user_input:.2} to degrees C from F...");
        output = (user_input - 32.0) * (5.0 / 9.0);
        println!("Result: {output:.2} degrees C")
    } else {
        println!("Converting {user_input:.2} to degrees F from C...");
        output = (user_input * (9.0 / 5.0)) + 32.0;
        println!("Result: {output:.2} degrees F");
    }

    // Fib sequence
    let agree: String = loop {
        println!("Would you like to see a fibbonacci sequence? (y/n)");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed: String = input.trim().to_lowercase();

        if trimmed == "y" || trimmed == "n" {
            break trimmed.to_string();
        } else {
            println!("Please enter y for yes or n for no");
        }
    };

    if agree == "y" {
        println!("Okay! Here's the first fifteen numbers of the Fibbonacci sequence!");
        let mut counter = 0;
        for n in 1..16 {
            counter += 1;
            let output = fib(n);
            println!("{counter}: {output}");
        }
    } else {
        println!("Alright! that's okay.")
    }
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
