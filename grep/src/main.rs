use grep::{run, Config};
use std::{env, process};


fn main() {
    // args.push(String::from("bOdY"));
    // args.push(String::from("poem.txt"));

    let config = Config::from(env::args()).unwrap_or_else(|e| {
        eprintln!("problem parsing arguments:{}", e);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}