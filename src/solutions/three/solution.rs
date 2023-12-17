use std::{collections::HashSet, fmt, ops::Index};

use crate::{error::SolutionError, util::SolutionPart};
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
                if row == point.row && col == point.col {
                    continue;
                }
                neighbors.push(SchematicPoint { row, col });
            }
        }

        neighbors
    }

    fn get_surrounding_numbers(&self, point: SchematicPoint) -> Vec<i32> {
        let mut explored_set: HashSet<SchematicPoint> = HashSet::new();
        let mut numbers: Vec<i32> = vec![];
        for neighbor in self.get_neighboring_points(point) {
            println!("{}", neighbor);
            if explored_set.contains(&neighbor) {
                continue;
            }
            if NUMBERS.contains(&self[neighbor]) {
                let mut pointer = neighbor.clone();
                loop {
                    if pointer.col == self.width || !NUMBERS.contains(&self[pointer]) {
                        pointer.col -= 1;
                        break;
                    }
                    pointer.col += 1;
                }
                let mut number = 0;
                let mut pow = 0;
                while let Some(digit) = self[pointer].to_digit(10) {
                    explored_set.insert(pointer.clone());
                    number += digit as i32 * 10_i32.pow(pow);
                    pow += 1;
                    if pointer.col == 0 {
                        break;
                    }
                    pointer.col -= 1;
                }
                println!("Pushing: {}", number);
                numbers.push(number);
            }
        }
        numbers
    }
}

fn part_one_solution(engine_schematic: EngineSchematic) -> i32 {
    let mut answer_sum: u32 = 0;

    for row in 0..engine_schematic.height {
        let mut pointer = SchematicPoint {
            row,
            col: engine_schematic.width - 1,
        };
        loop {
            if NUMBERS.contains(&engine_schematic[pointer]) {
                let mut found_symbol = false;
                let mut num = 0;
                let mut pow = 0;

                while let Some(digit) = &engine_schematic[pointer].to_digit(10) {
                    num += digit * 10_u32.pow(pow);
                    pow += 1;
                    for neighbor in engine_schematic.get_neighboring_points(pointer) {
                        if !found_symbol
                            && !NUMBERS.contains(&engine_schematic[neighbor])
                            && IGNORED_CELL != engine_schematic[neighbor]
                        {
                            found_symbol = true;
                        }
                    }
                    if pointer.col == 0 {
                        break;
                    }
                    pointer.col -= 1
                }
                if found_symbol {
                    answer_sum += num;
                }
            }
            if pointer.col == 0 {
                break;
            }
            pointer.col -= 1;
        }
    }
    answer_sum as i32
}

fn part_two_solution(engine_schematic: EngineSchematic) -> i32 {
    let mut answer: i32 = 0;
    for row in 0..engine_schematic.height {
        let mut pointer = SchematicPoint {
            row,
            col: engine_schematic.width - 1,
        };
        loop {
            if engine_schematic[pointer] == '*' {
                let surrounding_numbers = engine_schematic.get_surrounding_numbers(pointer);
                if surrounding_numbers.len() == 2 {
                    answer +=
                        surrounding_numbers.get(0).unwrap() * surrounding_numbers.get(1).unwrap();
                }
            }
            if pointer.col == 0 {
                break;
            }
            pointer.col -= 1;
        }
    }
    answer
}

pub fn run(problem_data: String, solution_part: SolutionPart) -> Result<i32, SolutionError> {
    let engine_schematic = EngineSchematic::from(problem_data);
    match solution_part {
        SolutionPart::PartOne => Ok(part_one_solution(engine_schematic)),
        SolutionPart::PartTwo => Ok(part_two_solution(engine_schematic)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SCHEMATIC: &str = ".*23\n45..\n....\n.*33";

    #[test]
    fn test_schematic_from_string() {
        let engine_schematic = EngineSchematic::from(TEST_SCHEMATIC.to_string());

        assert!(engine_schematic.height == 4);
        assert!(engine_schematic.width == 4);
        assert!(engine_schematic.data[0][2] == '2');
    }

    #[test]
    fn test_center_neighbor_points() {
        let engine_schematic = EngineSchematic::from(TEST_SCHEMATIC.to_string());

        let center_neighbor_points =
            engine_schematic.get_neighboring_points(SchematicPoint { col: 1, row: 1 });
        assert!(center_neighbor_points.contains(&SchematicPoint { row: 0, col: 0 }));
        assert!(center_neighbor_points.contains(&SchematicPoint { row: 0, col: 1 }));
        assert!(center_neighbor_points.contains(&SchematicPoint { row: 0, col: 2 }));
        assert!(center_neighbor_points.contains(&SchematicPoint { row: 1, col: 0 }));
        assert!(center_neighbor_points.contains(&SchematicPoint { row: 1, col: 2 }));
        assert!(center_neighbor_points.contains(&SchematicPoint { row: 2, col: 0 }));
        assert!(center_neighbor_points.contains(&SchematicPoint { row: 2, col: 1 }));
        assert!(center_neighbor_points.contains(&SchematicPoint { row: 2, col: 2 }));

        assert!(!center_neighbor_points.contains(&SchematicPoint { row: 1, col: 1 }));
    }

    #[test]
    fn test_edge_neighbor_points() {
        let engine_schematic = EngineSchematic::from(TEST_SCHEMATIC.to_string());

        let edge_neighbor_points =
            engine_schematic.get_neighboring_points(SchematicPoint { row: 1, col: 0 });
        assert!(edge_neighbor_points.contains(&SchematicPoint { row: 0, col: 0 }));
        assert!(edge_neighbor_points.contains(&SchematicPoint { row: 0, col: 1 }));
        assert!(edge_neighbor_points.contains(&SchematicPoint { row: 1, col: 1 }));
        assert!(edge_neighbor_points.contains(&SchematicPoint { row: 2, col: 0 }));
        assert!(edge_neighbor_points.contains(&SchematicPoint { row: 2, col: 1 }));

        assert!(!edge_neighbor_points.contains(&SchematicPoint { row: 0, col: 2 }));
        assert!(!edge_neighbor_points.contains(&SchematicPoint { row: 1, col: 2 }));
        assert!(!edge_neighbor_points.contains(&SchematicPoint { row: 2, col: 2 }));
        assert!(!edge_neighbor_points.contains(&SchematicPoint { row: 1, col: 0 }));
    }

    #[test]
    fn test_top_neighbor_points() {
        let engine_schematic = EngineSchematic::from(TEST_SCHEMATIC.to_string());

        let top_neighbor_points =
            engine_schematic.get_neighboring_points(SchematicPoint { row: 0, col: 1 });

        assert!(top_neighbor_points.contains(&SchematicPoint { row: 0, col: 0 }));
        assert!(top_neighbor_points.contains(&SchematicPoint { row: 0, col: 2 }));
        assert!(top_neighbor_points.contains(&SchematicPoint { row: 1, col: 0 }));
        assert!(top_neighbor_points.contains(&SchematicPoint { row: 1, col: 1 }));
        assert!(top_neighbor_points.contains(&SchematicPoint { row: 1, col: 2 }));

        assert!(!top_neighbor_points.contains(&SchematicPoint { row: 0, col: 1 }));
    }
}
