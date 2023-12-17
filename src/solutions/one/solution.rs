use crate::{error::SolutionError, util::SolutionPart};

const NUMBER_WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn number_word_to_i32(word: &str) -> u32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

fn extract_number_word(s: &str, start_index: usize) -> Option<u32> {
    for word in NUMBER_WORDS {
        if start_index as i32 - word.len() as i32 + 1 < 0 {
            continue;
        }
        let word_slice = &s[start_index + 1 - word.len()..start_index + 1];
        if word_slice == *word {
            return Some(number_word_to_i32(word));
        }
    }
    None
}

fn extract_number(s: &str, reverse: bool, accept_num_words: bool) -> Result<u32, SolutionError> {
    for i in 0..s.len() {
        let index = if reverse { s.len() - i - 1 } else { i };
        let c = s.chars().nth(index).unwrap();
        if let Some(digit_char) = c.to_digit(10) {
            return Ok(digit_char);
        }
        if accept_num_words {
            if let Some(digit_char) = extract_number_word(s, index) {
                return Ok(digit_char);
            }
        }
    }
    Err(SolutionError::NoPossibleSolution)
}

pub fn run(problem_data: String, part: SolutionPart) -> Result<String, SolutionError> {
    let mut calibration_sum: u32 = 0;

    let accept_num_words = match part {
        SolutionPart::PartOne => false,
        SolutionPart::PartTwo => true,
    };

    for scribble in problem_data.split('\n') {
        let first_digit: u32 = extract_number(scribble, false, accept_num_words)?;
        let second_digit: u32 = extract_number(scribble, true, accept_num_words)?;
        calibration_sum += (first_digit * 10) + second_digit;
    }
    Ok(calibration_sum.to_string())
}
