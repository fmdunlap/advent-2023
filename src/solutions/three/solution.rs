use std::{fmt, ops::Index, path::PathBuf};

use crate::{
    error::SolutionError,
    util::{load_file, SolutionPart},
};
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const IGNORED_CELL: char = '.';

struct EngineSchematic {
    height: usize,
    width: usize,
    data: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct SchematicPoint {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct NoSuchCellError {
    point: SchematicPoint,
}

impl fmt::Display for NoSuchCellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cell {{Row: {}, Col: {}}} not found.",
            self.point.row, self.point.col
        )
    }
}

impl fmt::Display for SchematicPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}", self.row, self.col)
    }
}

impl Index<usize> for EngineSchematic {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<SchematicPoint> for EngineSchematic {
    type Output = char;

    fn index(&self, index: SchematicPoint) -> &Self::Output {
        &self.data[index.row][index.col]
    }
}

impl From<String> for EngineSchematic {
    fn from(value: String) -> Self {
        let data: Vec<Vec<char>> = value.split('\n').map(|str| str.chars().collect()).collect();
        let height = data.len();
        let width = data[0].len();

        EngineSchematic {
            height,
            width,
            data,
        }
    }
}

impl EngineSchematic {
    fn get_neighboring_points(&self, point: SchematicPoint) -> Vec<SchematicPoint> {
        let mut neighbors: Vec<SchematicPoint> = vec![];

        let min_row: usize = match point.row.checked_sub(1) {
            Some(min_row) => min_row,
            None => 0,
        };
        let min_col: usize = match point.col.checked_sub(1) {
            Some(min_col) => min_col,
            None => 0,
        };
        let max_row: usize = if point.row + 1 < self.height {
            point.row + 1
        } else {
            point.row
        };
        let max_col: usize = if point.col + 1 < self.width {
            point.col + 1
        } else {
            point.col
        };

        for row in min_row..max_row + 1 {
            for col in min_col..max_col + 1 {
                neighbors.push(SchematicPoint { row, col });
            }
        }

        neighbors
    }

    fn get_symbol_points(&self) -> Vec<SchematicPoint> {
        let mut symbol_cells = vec![];

        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self[row][col];
                if cell == IGNORED_CELL || NUMBERS.contains(&cell) {
                    continue;
                }
                symbol_cells.push(SchematicPoint { row, col });
            }
        }
        symbol_cells
    }

    fn get_full_number_points(&self, point: SchematicPoint) -> Vec<SchematicPoint> {
        let mut pointer = point.clone();
        let mut number_points = vec![];
        while pointer.col < self.width - 1 {
            if !NUMBERS.contains(&self[pointer.row][pointer.col + 1]) {
                break;
            }
            pointer.col += 1;
        }
        while pointer.col > 0 {
            number_points.push(pointer);
            if !NUMBERS.contains(&self[pointer.row][pointer.col - 1]) {
                break;
            }
            pointer.col -= 1;
        }
        number_points
    }
}

pub fn run(data_path: PathBuf, solution_part: SolutionPart) -> Result<i32, SolutionError> {
    let engine_schematic = EngineSchematic::from(load_file(data_path)?);
    let mut answer_sum: i32 = 0;

    let mut candidate_numbers: Vec<Vec<SchematicPoint>> = vec![];

    for symbol in engine_schematic.get_symbol_points() {
        let mut explored_number_points: Vec<SchematicPoint> = vec![];
        for neighbor in engine_schematic.get_neighboring_points(symbol) {
            if explored_number_points.contains(&neighbor) {
                continue;
            }
            if NUMBERS.contains(&engine_schematic[neighbor]) {
                let number_points = engine_schematic.get_full_number_points(neighbor);
                candidate_numbers.push(number_points.clone());
                for number_point in number_points {
                    explored_number_points.push(number_point);
                }
            }
        }
    }

    for candidate_points in candidate_numbers {
        let mut number = 0;
        let mut pow: u32 = 0;
        let mut i = 0;
        while let Some(digit) = engine_schematic[candidate_points[i]].to_digit(10) {
            number += digit as i32 * 10_i32.pow(pow);
            pow += 1;
            i += 1;
            if i >= candidate_points.len() {
                break;
            }
        }
        if candidate_points[0].row >= 135 {
            println!("{}", number);
        }
        answer_sum += number;
    }

    let _ = solution_part;
    Ok(answer_sum)
}
