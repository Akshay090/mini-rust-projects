use clap::Parser;
use std::fs;

// search for a pattern in a file and display the lines containg it
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // Path of the file to read
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();

    let contants = fs::read_to_string(args.path).expect("could not read file");

    for line in contants.lines() {
        if line.contains(&args.pattern) {
            print!("{}", line);
        }
    }
}
