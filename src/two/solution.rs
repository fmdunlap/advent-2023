use std::cmp::max;
use std::path::PathBuf;

use crate::{error::SolutionError, util::load_file, util::SolutionPart};

struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    index: i32,
    draws: Vec<CubeSet>,
}

fn extract_rolls(draw: &str, color: &str) -> Result<i32, SolutionError> {
    match draw.replace(color, "").trim().parse::<i32>() {
        Ok(drawn_color) => Ok(drawn_color),
        Err(_) => Err(SolutionError::NoPossibleSolution),
    }
}

fn extract_game_parts(game_str: &str) -> Result<Game, SolutionError> {
    let mut split_game_str = game_str.split(':');
    let game_index = split_game_str
        .nth(0)
        .unwrap()
        .replace("Game ", "")
        .parse::<i32>()
        .unwrap();
    let mut draws: Vec<CubeSet> = vec![];

    for draw_set in split_game_str.nth(0).unwrap().split(";") {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for draw in draw_set.split(',') {
            let draw = draw.trim();
            if draw.ends_with("red") {
                red = extract_rolls(draw, "red")?;
            } else if draw.ends_with("green") {
                green = extract_rolls(draw, "green")?;
            } else if draw.ends_with("blue") {
                blue = extract_rolls(draw, "blue")?;
            }
        }
        draws.push(CubeSet { red, green, blue })
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

fn game_cubeset_power(game: &Game) -> i32 {
    let mut min_cube_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for draw in game.draws.as_slice() {
        min_cube_set.red = max(min_cube_set.red, draw.red);
        min_cube_set.green = max(min_cube_set.green, draw.green);
        min_cube_set.blue = max(min_cube_set.blue, draw.blue);
    }
    min_cube_set.red * min_cube_set.green * min_cube_set.blue
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
            SolutionPart::PartTwo => {
                puzzle_answer += game_cubeset_power(&game);
            }
        }
    }

    Ok(puzzle_answer)
}
