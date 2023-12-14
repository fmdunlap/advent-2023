mod error;
mod solutions;
mod util;
use clap::Parser;
use error::SolutionError;
use std::path::PathBuf;
use util::SolutionPart;

#[derive(Parser)]
struct Cli {
    problem: String,
    part: String,
    data_path: PathBuf,
}

fn print_error_message(err_type: SolutionError) {
    let error_message = match err_type {
        SolutionError::NoPossibleSolution => {
            String::from("No solution is possible given the provided data.")
        }
        SolutionError::FileLoadError => String::from("File failed to load."),
    };
    println!("Failed to compute solution with error: {}", error_message);
}

fn print_solved_message(answer: String) {
    println!("Got an answer! Answer is: {}", answer);
}

fn main() {
    let args = Cli::parse();

    let solution_part = match args.part.as_str() {
        "1" => SolutionPart::PartOne,
        "2" => SolutionPart::PartTwo,
        _ => {
            println!("Solution part was invalid!");
            return;
        }
    };

    match args.problem.as_str() {
        "1" => match solutions::one::run(args.data_path) {
            Ok(answer) => print_solved_message(answer.to_string()),
            Err(err) => print_error_message(err),
        },
        "2" => match solutions::two::run(args.data_path, solution_part) {
            Ok(answer) => print_solved_message(answer.to_string()),
            Err(err) => print_error_message(err),
        },
        _ => {
            println!("Not 1")
        }
    }
}
