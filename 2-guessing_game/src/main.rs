

use rand::Rng;
use std::{ io, cmp::Ordering };


fn main() {
    println!("Guessing Game");

    let rand_num: u8 = rand::thread_rng().gen_range(1..100);
    let mut counter = 0;


    loop {
        counter += 1;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer between 1 and 100.");
                counter -= 1;
                continue;

            }
        };

        // if &guess > &rand_num {
        //     println!("Too high!");
        // } else if &guess < &rand_num {
        //     println!("Too low!");
        // } else if &guess == &rand_num {
        //     println!("You got it in {} tries!", counter);
        //     break;
        // }

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
