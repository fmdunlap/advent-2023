use crate::{error::SolutionError, util::SolutionPart};

struct DataPayload {
    _races: Vec<(i32, i32)>,
}

fn parse_input_data(_problem_data: String) -> Result<DataPayload, SolutionError> {
    Ok(DataPayload { _races: vec![] })
}

fn solve(_payload: DataPayload) -> i32 {
    0
}

pub fn run(problem_data: String, _solution_part: SolutionPart) -> Result<String, SolutionError> {
    let payload = parse_input_data(problem_data)?;

    Ok(solve(payload).to_string())
}
