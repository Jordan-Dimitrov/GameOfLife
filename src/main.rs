use std::{env, process};
use game_of_life::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{0}", config.height);
    println!("{0}", config.width);
}
