#[derive(Debug)]
pub enum SolutionError {
    NoPossibleSolution,
    FileLoadError,
    DataParsingError,
    UnknownProblem,
}
