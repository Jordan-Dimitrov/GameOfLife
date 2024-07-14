use std::{env, process};
use game_of_life::{Config, Game};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut game = Game::new(config);

    game.run();
}
