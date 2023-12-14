use std::error::Error;
use std::path::PathBuf;

const NUMBER_WORDS: &'static [&'static str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub enum SolutionError {
    NoNumberFound,
    FileLoadError,
}

fn load_file(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let data = std::fs::read_to_string(path)?;
    return Ok(data);
}

fn number_word_to_i32(word: &str) -> i32 {
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

fn extract_number_word(s: &str, start_index: usize) -> Option<i32> {
    for word in NUMBER_WORDS {
        if start_index as i32 - word.len() as i32 + 1 < 0 {
            continue;
        }
        let word_slice = &s[start_index + 1 - word.len()..start_index + 1];
        if word_slice == *word {
            return Some(number_word_to_i32(word));
        }
    }
    return None;
}

fn extract_number(s: &str, reverse: bool) -> Result<i32, SolutionError> {
    for i in 0..s.len() {
        let index = if reverse { s.len() - i - 1 } else { i };
        let c = s.chars().nth(index).unwrap();
        if let Some(digit_char) = c.to_digit(10) {
            return Ok(digit_char as i32);
        }
        if let Some(digit_char) = extract_number_word(s, index) {
            return Ok(digit_char);
        }
    }
    return Err(SolutionError::NoNumberFound);
}

pub fn run(data_path: PathBuf) -> Result<i32, SolutionError> {
    if let Ok(contents) = load_file(data_path) {
        let mut calibration_sum: i32 = 0;

        for scribble in contents.split('\n') {
            let first_digit: i32 = extract_number(scribble, false)?;
            let second_digit: i32 = extract_number(scribble, true)?;
            calibration_sum += (first_digit * 10) + second_digit;
        }
        return Ok(calibration_sum);
    } else {
        return Err(SolutionError::FileLoadError);
    }
}
