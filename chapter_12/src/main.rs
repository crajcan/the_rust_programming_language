extern crate chapter_12;

use std::env;
use std::process;

use chapter_12::Config;

fn main() {
    let mut writer = std::io::stdout();

    let config = Config::new(env::args(), &mut writer).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = chapter_12::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
