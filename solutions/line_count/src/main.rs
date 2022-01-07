use std::fs::File;
use std::io::BufRead;
use std::process;
use std::{env, io};

fn count_lines(file_path: &str) -> std::io::Result<usize> {
    let file = File::open(file_path)?;

    Ok(io::BufReader::new(file).lines().count())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file path>", args[0]);

        process::exit(0x1);
    }

    if let Ok(line_count) = count_lines(&args[1]) {
        println!("{} contains {} line(s)", args[1], line_count);
    } else {
        println!("Failed to read from {}", args[1]);
    }
}
