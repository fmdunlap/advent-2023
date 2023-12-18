mod error;
mod server;
mod solutions;
mod util;
use clap::Parser;
use error::SolutionError;
use std::path::PathBuf;
use util::{load_file, SolutionPart};

#[derive(Parser)]
struct Cli {
    command: String,
    problem: String,
    part: String,
    data_path: PathBuf,
}

fn print_error_message(err_type: &SolutionError) {
    let error_message = match err_type {
        SolutionError::NoPossibleSolution => {
            String::from("No solution is possible given the provided data.")
        }
        SolutionError::FileLoadError => String::from("File failed to load."),
        SolutionError::UnknownProblem => String::from("Unknown Problem"),
        SolutionError::DataParsingError => {
            String::from("An error occurred while parsing the provided data.")
        }
    };
    println!("Failed to compute solution with error: {}", error_message);
}

fn print_solved_message(answer: String) {
    println!("Got an answer! Answer is: {}", answer);
}

fn run_and_print(problem: u32, part: SolutionPart, data_path: PathBuf) {
    let problem_result = load_file(data_path);

    if problem_result.is_err() {
        print_error_message(problem_result.as_ref().err().unwrap());
    }

    let problem_data = problem_result.unwrap();

    match problem {
        1 => match solutions::one::run(problem_data, part) {
            Ok(answer) => print_solved_message(answer),
            Err(err) => print_error_message(&err),
        },
        2 => match solutions::two::run(problem_data, part) {
            Ok(answer) => print_solved_message(answer),
            Err(err) => print_error_message(&err),
        },
        3 => match solutions::three::run(problem_data, part) {
            Ok(answer) => print_solved_message(answer),
            Err(err) => print_error_message(&err),
        },
        4 => match solutions::four::run(problem_data, part) {
            Ok(answer) => print_solved_message(answer),
            Err(err) => print_error_message(&err),
        },
        5 => match solutions::five::run(problem_data, part) {
            Ok(answer) => print_solved_message(answer),
            Err(err) => print_error_message(&err),
        },
        6 => match solutions::six::run(problem_data, part) {
            Ok(answer) => print_solved_message(answer),
            Err(err) => print_error_message(&err),
        },
        _ => print_error_message(&SolutionError::UnknownProblem),
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let solution_part = match args.part.as_str() {
        "1" => SolutionPart::PartOne,
        "2" => SolutionPart::PartTwo,
        _ => {
            println!("Solution part was invalid!");
            return;
        }
    };

    match args.command.as_str() {
        "run" => run_and_print(
            args.problem.parse::<u32>().unwrap(),
            solution_part,
            args.data_path,
        ),
        "serve" => server::serve(None, None).await,
        _ => println!("Command malformed."),
    }
}
