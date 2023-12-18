use std::{net::Ipv4Addr, str::FromStr};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{error::SolutionError, solutions, util::SolutionPart};

const DEFAULT_SERVER_ADDR: &str = "0.0.0.0";
const DEFAULT_SERVER_PORT: u16 = 3000_u16;

pub async fn serve(addr: Option<Ipv4Addr>, port: Option<u16>) {
    let server_listen_addr = addr.unwrap_or(Ipv4Addr::from_str(DEFAULT_SERVER_ADDR).unwrap());
    let server_listen_port = port.unwrap_or(DEFAULT_SERVER_PORT);

    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/solve", post(solve));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        server_listen_addr.to_string(),
        server_listen_port
    ))
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn get_solution_part(payload: &ProblemInput) -> Option<SolutionPart> {
    match payload.part {
        1 => Some(SolutionPart::PartOne),
        2 => Some(SolutionPart::PartTwo),
        _ => None,
    }
}

fn get_problem_answer(
    problem: u8,
    part: SolutionPart,
    input: String,
) -> Result<String, SolutionError> {
    let input = input.replace('`', "\n");
    match problem {
        1 => solutions::one::run(input, part),
        2 => solutions::two::run(input, part),
        3 => solutions::three::run(input, part),
        4 => solutions::four::run(input, part),
        5 => solutions::five::run(input, part),
        _ => Err(SolutionError::UnknownProblem),
    }
}

async fn solve(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<ProblemInput>,
) -> (StatusCode, Json<ProblemOutput>) {
    let part = get_solution_part(&payload);
    if part.is_none() {
        let response: ProblemOutput = ProblemOutput {
            answer: None,
            error: Some(format!(
                "Provided part number {} is not 1 or 2.",
                payload.part
            )),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    let part = part.unwrap();

    match get_problem_answer(payload.problem, part, payload.data) {
        Ok(problem_answer) => {
            let answer: ProblemOutput = ProblemOutput {
                answer: Some(problem_answer),
                error: None,
            };
            (StatusCode::OK, Json(answer))
        }
        Err(problem_error) => {
            let error_message = match problem_error {
                SolutionError::NoPossibleSolution => "Solution could not be computed",
                SolutionError::FileLoadError => "Failed to get solution input data",
                SolutionError::UnknownProblem => "Unknown error occurred",
                SolutionError::DataParsingError => {
                    "An error occured while parsing the provided data."
                }
            };
            let problem_output = ProblemOutput {
                answer: None,
                error: Some(error_message.to_string()),
            };
            (StatusCode::BAD_REQUEST, Json(problem_output))
        }
    }
}

#[derive(Deserialize)]
struct ProblemInput {
    data: String,
    problem: u8,
    part: u8,
}

#[derive(Serialize)]
struct ProblemOutput {
    answer: Option<String>,
    error: Option<String>,
}
