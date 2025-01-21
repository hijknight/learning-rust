use std::error::Error;
use std::{fs, process};
use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.contains(&String::from("help")) {
            help();
        } else if args.len() < 3 {
            return Err("not enough arguments\nUsage: `cargo run -- query file_path`\n\nrun `cargo run -- help` for more info");
        }

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();


        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    if config.ignore_case {
        let results = search_insensitive(&config.query, &contents);

        if results.len() == 0 {
            println!("No lines containing '{}' in file {}.", &config.query, &config.file_path);
        } else {
            for line in results {
                println!("{line}");
            }
        }
    } else {
        let results = search(&config.query, &contents);

        if results.len() == 0 {
            println!("No lines containing '{}' in file {}.", &config.query, &config.file_path)
        } else {
            for line in results {
                println!("{line}");
            }
        }

    }

    Ok(())

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

fn help() {
    println!("Run program with `cargo run -- query file_path`.");
    println!("To open this help page, run `cargo run -- help`.");
    println!();
    println!("In order to ignore case add `IGNORE_CASE = 1` env variable in front of command, like so:");
    println!("`IGNORE_CASE = 1 cargo run -- query file_path`");
    println!();
    println!("   Program desc.:");
    println!("     Parses through each line of a given file and return the line(s) containing a query word.");
    process::exit(2);
}

mod tests;