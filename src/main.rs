use std::process;
use clap::Parser;

fn main() {
    let config = anime_gif_generator::Config::parse();

    if let Err(e) = anime_gif_generator::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}