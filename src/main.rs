mod error;
mod one;
mod two;
mod util;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    problem: String,
    data_path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    match args.problem.as_str() {
        "1" => {
            if let Ok(sum) = one::run(args.data_path) {
                println!("Sum: {}", sum);
            } else {
                println!("No sum");
            }
        }
        _ => {
            println!("Not 1")
        }
    }
}
