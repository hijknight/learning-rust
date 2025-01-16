use rand::Rng;
use std::{ io, cmp::Ordering };


fn main() {
    println!("Guessing Game");


    let rand_num: i32 = rand::thread_rng().gen_range(1..100);
    let mut counter = 0;


    loop {
        if counter == 0 {
            println!("Please enter a guess between 1 and 100: ")
        } else {
            println!("Guess again!: ")
        }
        counter += 1;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer between 1 and 100.");
                counter -= 1;
                continue;
            }
        };
        // adds a check for guess between 1 and 100
        if guess < 0 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }


        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it in {counter} tries!");
                break;
            }
        }
    }
}

