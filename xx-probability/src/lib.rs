use rand::Rng;
use std::{env, process};

pub struct Config {
    pub num_flips: i32,
    pub iterations: i32,
    pub test: bool,
}

pub fn run(config: Config) {
    let num_flips = config.num_flips;
    let iterations = config.iterations;

    if !config.test {
        let mut rng = rand::thread_rng();
        let mut total_tries = 0;

        for _ in 0..iterations {
            let mut counter = 0;
            let mut perfect = false;

            while !perfect {
                counter += 1;
                let mut heads = 0;
                let mut tails = 0;

                for _ in 0..num_flips {
                    let flip: i32 = rng.gen_range(1..=2);
                    if flip == 1 {
                        heads += 1;
                    } else {
                        tails += 1;
                    }
                }

                if heads == tails {
                    perfect = true;
                }
            }

            println!("Got a perfect 50/50 in {} tries", counter);
            total_tries += counter;
        }

        let avg_tries = total_tries as f64 / iterations as f64;
        println!("Average tries until perfect is: {}", avg_tries);
        println!("Tested {} times", iterations);
    } else {
        let mut rng = rand::thread_rng();
        for _ in 0..iterations {
            let mut heads = 0;
            let mut tails = 0;

            for _ in 0..num_flips {
                let flip: i32 = rng.gen_range(1..=2);
                if flip == 1 {
                    heads += 1;
                } else {
                    tails += 1;
                }
            }

            let proportion_heads = heads as f64 / (heads + tails) as f64;
            println!("Proportion heads: {}", proportion_heads);
        }
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let num_flips: i32 = args[1].clone().parse().unwrap_or_else(|err| {
            println!("please enter a valid number for number of coin flips: {err}");
            process::exit(1);
        });

        let iterations: i32 = if args.len() > 2 {
            args[2].clone().parse().unwrap_or_else(|err| {
                println!("please enter a valid for number of iterations: {err}");
                process::exit(1);
            })
        } else {
            1
        };
        let test = env::var("TEST").is_ok();

        Ok(Config {
            num_flips,
            iterations,
            test,
        })
    }
}
