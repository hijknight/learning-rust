use std::{ env, process };
use rand::Rng;

pub struct Config {
    pub num_flips: i32,
    pub iterations: i32,
    pub use_perfect: bool,
}

pub fn run(config: Config) {
    let num_flips = config.num_flips;
    let iterations = config.iterations;



    for _ in 0..iterations {
        let mut perfect: i32 = 0;
        let mut counter: i32 = 0;
        if config.use_perfect {
            while perfect < 1 {

                counter += 1;
                // println!("\nRun: {}", counter);
                let mut tails: i32 = 0;
                let mut heads: i32 = 0;

                for _ in 0..num_flips {
                    let flip: i32 = rand::thread_rng().gen_range(1..=2);

                    if flip == 1 {
                        heads += 1;
                    } else {
                        tails += 1;
                    }
                }
                // println!("Heads: {}", heads);
                // println!("Tails: {}", tails);
                let probability = heads as f64 / (heads as f64 + tails as f64);
                if probability == 0.5 {
                    perfect += 1;
                }
                println!("Run {}: {}", counter, probability);
            }

            println!("Got a perfect in {} tries", counter)
        } else {
            let mut tails: i32 = 0;
            let mut heads: i32 = 0;

            for _ in 0..num_flips {
                let flip: i32 = rand::thread_rng().gen_range(1..=2);

                if flip == 1 {
                    heads += 1;
                } else {
                    tails += 1;
                }
            }
            // println!("Heads: {}", heads);
            // println!("Tails: {}", tails);
            let probability = heads as f64 / (heads as f64 + tails as f64);
            println!("Probability with {}: {}", num_flips, probability);
        }

    }

}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments")
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

        let use_perfect = env::var("USE_PERFECT").is_ok();

        Ok(Config { num_flips, iterations, use_perfect })
    }
}
