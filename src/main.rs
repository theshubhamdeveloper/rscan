use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <path>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let path = &args[2];

    println!("Searching for {}", pattern);
    println!("In file {}", path);

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");

        if line.contains(pattern) {
            println!("{}: {}", index + 1, line)
        };
    }
}
