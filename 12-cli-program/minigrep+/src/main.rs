use minigrep_plus::Config;
use std::env;
use std::process;


fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("minigrep+: error reading args: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_plus::run(config) {
        eprintln!("minigrep+: application Error: {e}");
        process::exit(3);
    }
}

