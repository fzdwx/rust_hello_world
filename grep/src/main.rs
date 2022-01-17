use grep::{run, Config};
use std::{env, process};


fn main() {
    let args: Vec<String> = env::args().collect();
    // args.push(String::from("bOdY"));
    // args.push(String::from("poem.txt"));

    let config = Config::from(&args).unwrap_or_else(|e| {
        eprintln!("problem parsing arguments:{}", e);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}