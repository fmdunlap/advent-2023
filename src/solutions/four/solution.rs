use std::path::PathBuf;

use crate::{
    error::SolutionError,
    util::{load_file, SolutionPart},
};

pub fn run(data_path: PathBuf, solution_part: SolutionPart) -> Result<i32, SolutionError> {
    let content = load_file(data_path)?;
    match solution_part {
        SolutionPart::PartOne => Ok(part_one_solution(content)),
        SolutionPart::PartTwo => Ok(part_two_solution(content)),
    }
}

fn part_two_solution(content: String) -> i32 {
    todo!()
}

fn part_one_solution(content: String) -> i32 {
    todo!()
}
