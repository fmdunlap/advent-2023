use crate::{error::SolutionError, util::SolutionPart};
use arr_macro::arr;

// The correct (optimized) approach for this problem is actually a kinda interesting
// tree. See https://en.wikipedia.org/wiki/Interval_tree for details, but my approach
// is as follows.
//
// Create an interval tree

#[derive(Clone, Copy)]
struct AlmanacRange {
    start: i64,
    end: i64,
    transform_factor: i64,
}

#[derive(Clone, Default)]
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl AlmanacRange {
    fn contains(&self, i: i64) -> bool {
        i >= self.start && i < self.end
    }
}

struct DataPayload {
    seeds: Vec<i64>,
    maps: [AlmanacMap; 7],
}

impl From<&str> for AlmanacRange {
    fn from(value: &str) -> Self {
        let mut values = value.split(' ');
        let dest_start = values.nth(0).unwrap().parse::<i64>().unwrap();
        let source_start = values.nth(0).unwrap().parse::<i64>().unwrap();
        let length = values.nth(0).unwrap().parse::<i64>().unwrap();
        AlmanacRange {
            start: source_start,
            end: source_start + length,
            transform_factor: dest_start - source_start,
        }
    }
}

impl AlmanacMap {
    fn get_transform(&self, i: i64) -> i64 {
        for range in &self.ranges {
            if range.contains(i) {
                return range.transform_factor;
            }
        }
        return 0;
    }

    fn builder() -> Self {
        AlmanacMap { ranges: vec![] }
    }
}

fn extract_seeds(seeds_str: &str, solution_part: &SolutionPart, container: &mut Vec<i64>) {
    let seeds_vec: Vec<i64> = seeds_str
        .trim()
        .split(' ')
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();
    match solution_part {
        SolutionPart::PartOne => {
            for seed in seeds_vec {
                container.push(seed)
            }
        }
        SolutionPart::PartTwo => {
            for pair in seeds_vec.chunks_exact(2) {
                let start = pair[0];
                let range = pair[1];
                for i in start..range {
                    container.push(i);
                }
            }
        }
    }
}

fn parse_input_data(
    problem_data: &String,
    solution_part: SolutionPart,
) -> Result<DataPayload, SolutionError> {
    let mut seeds: Vec<i64> = vec![];
    let mut maps: [AlmanacMap; 7] = arr![AlmanacMap::builder(); 7];

    let chunks = problem_data.split("\n\n");

    for chunk in chunks {
        let mut lines = chunk.lines();
        let title_line = lines.nth(0).unwrap();
        if !title_line.contains(':') {
            return Err(SolutionError::DataParsingError);
        }
        if let Some(title) = title_line.split(':').nth(0) {
            match title {
                "seeds" => {
                    if let Some(seed_str) = title_line.split(':').nth(1) {
                        extract_seeds(seed_str, &solution_part, &mut seeds);
                    }
                    continue;
                }
                "seed-to-soil map" => {
                    for range_str in lines {
                        maps[0].ranges.push(AlmanacRange::from(range_str));
                    }
                }
                "soil-to-fertilizer map" => {
                    for range_str in lines {
                        maps[1].ranges.push(AlmanacRange::from(range_str));
                    }
                }
                "fertilizer-to-water map" => {
                    for range_str in lines {
                        maps[2].ranges.push(AlmanacRange::from(range_str));
                    }
                }
                "water-to-light map" => {
                    for range_str in lines {
                        maps[3].ranges.push(AlmanacRange::from(range_str));
                    }
                }
                "light-to-temperature map" => {
                    for range_str in lines {
                        maps[4].ranges.push(AlmanacRange::from(range_str));
                    }
                }
                "temperature-to-humidity map" => {
                    for range_str in lines {
                        maps[5].ranges.push(AlmanacRange::from(range_str));
                    }
                }
                "humidity-to-location map" => {
                    for range_str in lines {
                        maps[6].ranges.push(AlmanacRange::from(range_str));
                    }
                }
                _ => return Err(SolutionError::DataParsingError),
            }
        } else {
            return Err(SolutionError::DataParsingError);
        }
    }
    Ok(DataPayload { seeds, maps })
}

pub fn run(problem_data: String, solution_part: SolutionPart) -> Result<String, SolutionError> {
    let payload = parse_input_data(&problem_data, solution_part)?;

    Ok(solve(payload).to_string())
}

fn solve(payload: DataPayload) -> i64 {
    let mut minimum_location = i64::MAX;

    for (_index, seed) in payload.seeds.iter().enumerate() {
        let mut pointer_value = seed.clone();
        for map in payload.maps.as_slice() {
            pointer_value += map.get_transform(pointer_value);
        }
        minimum_location = i64::min(pointer_value, minimum_location);
    }

    minimum_location
}
