use std::{env,process};
use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let configs = Config::new(&args).unwrap_or_else(|err| {
        println!("problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(&configs) {
        println!("application error: {}", e);
        process::exit(1);
    }
}