use minigrep_plus::Config;
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect(); // creates an iterator from the args entered like so: cargo run -- arg1(query) arg2(file_path)

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("minigrep+: error reading args: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_plus::run(config) {
        eprintln!("minigrep+: application Error: {e}");
        process::exit(3);
    }
}

