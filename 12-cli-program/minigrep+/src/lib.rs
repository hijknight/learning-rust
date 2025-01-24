use std::error::Error;
use std::{fs, process};
use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let first_arg = args.next();
        let second_arg = args.next();
        if first_arg == Some(String::from("help")) {
            help();
        }


        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        let query = match first_arg {
            Some(arg) => arg,
            None => return Err("no argument found for query\nUsage: `cargo run -- (query) (file_path)`\n\nrun `cargo run -- help` for more info"),
        };
        let file_path = match second_arg {
            Some(arg) => arg,
            None => return Err("no argument found for file_path\nUsage: `cargo run -- query file_path`\n\nrun `cargo run -- help` for more info"),
        };

        Ok(Config {
            query,
            file_path,
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

fn help() {
    println!("Run program with `cargo run -- query file_path`.");
    println!("To open this help page, run `cargo run -- help`.");
    println!();
    println!("In order to ignore case add `IGNORE_CASE = 1` env variable in front of command, like so:");
    println!("`IGNORE_CASE = 1 cargo run -- query file_path`");
    println!();
    println!("Program desc.:");
    println!("     Parses through each line of a given file and return the line(s) containing a query word.");
    process::exit(2);
}

mod tests;