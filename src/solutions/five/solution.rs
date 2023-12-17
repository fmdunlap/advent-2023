use crate::{error::SolutionError, util::SolutionPart};

pub fn run(problem_data: String, solution_part: SolutionPart) -> Result<i32, SolutionError> {
    match solution_part {
        SolutionPart::PartOne => Ok(part_one_solution(problem_data)),
        SolutionPart::PartTwo => Ok(part_two_solution(problem_data)),
    }
}

fn part_two_solution(problem_data: String) -> i32 {
    todo!()
}

fn part_one_solution(problem_data: String) -> i32 {
    todo!()
}
