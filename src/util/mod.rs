use std::path::PathBuf;

use crate::error::SolutionError;

pub fn load_file(path: PathBuf) -> Result<String, SolutionError> {
    if let Ok(data) = std::fs::read_to_string(path) {
        Ok(data)
    } else {
        Err(SolutionError::FileLoadError)
    }
}
pub enum SolutionPart {
    PartOne,
    PartTwo,
}
