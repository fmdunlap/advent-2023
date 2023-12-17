use crate::{error::SolutionError, util::SolutionPart};

pub fn run(problem_data: String, solution_part: SolutionPart) -> Result<String, SolutionError> {
    match solution_part {
        SolutionPart::PartOne => Ok(part_one_solution(problem_data).to_string()),
        SolutionPart::PartTwo => Ok(part_two_solution(problem_data).to_string()),
    }
}

fn part_two_solution(problem_data: String) -> i32 {
    todo!()
}

fn part_one_solution(problem_data: String) -> i32 {
    todo!()
}
