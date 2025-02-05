use std::{env, process};
use probability::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("probability: problem parsing args: {err}"); // not enough args
        process::exit(1);
    });

    probability::run(config);
}



