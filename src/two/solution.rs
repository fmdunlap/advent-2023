use std::path::PathBuf;

use crate::{error::SolutionError, util::load_file, util::SolutionPart};

struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    index: i32,
    draws: Vec<Draw>,
}

fn extract_game_parts(game_str: &str) -> Result<Game, SolutionError> {
    let mut split_game_str = game_str.split(':');
    let game_index = split_game_str
        .nth(0)
        .unwrap()
        .replace("Game ", "")
        .parse::<i32>()
        .unwrap();
    let mut draws: Vec<Draw> = vec![];

    for draw_set in split_game_str.nth(0).unwrap().split(";") {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for draw in draw_set.split(',') {
            let draw = draw.trim();
            if draw.ends_with("red") {
                if let Ok(drawn_reds) = draw.replace(" red", "").parse::<i32>() {
                    red = drawn_reds;
                } else {
                    return Err(SolutionError::NoPossibleSolution);
                }
            }
            if draw.ends_with("green") {
                if let Ok(drawn_reds) = draw.replace(" green", "").parse::<i32>() {
                    green = drawn_reds;
                } else {
                    return Err(SolutionError::NoPossibleSolution);
                }
            }
            if draw.ends_with("blue") {
                if let Ok(drawn_reds) = draw.replace(" blue", "").parse::<i32>() {
                    blue = drawn_reds;
                } else {
                    return Err(SolutionError::NoPossibleSolution);
                }
            }
        }
        draws.push(Draw { red, green, blue })
    }

    return Ok(Game {
        index: game_index,
        draws: draws,
    });
}

fn game_is_valid(game: &Game) -> bool {
    for draw in game.draws.as_slice() {
        if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
            return false;
        }
    }
    return true;
}

pub fn run(data_path: PathBuf, solution_part: SolutionPart) -> Result<i32, SolutionError> {
    let contents = load_file(data_path)?;
    let mut puzzle_answer: i32 = 0;

    for game_str in contents.split('\n') {
        let game = extract_game_parts(game_str)?;

        match solution_part {
            SolutionPart::PartOne => {
                if game_is_valid(&game) {
                    puzzle_answer += game.index;
                }
            }
            SolutionPart::PartTwo => todo!(),
        }
    }

    Ok(puzzle_answer)
}
