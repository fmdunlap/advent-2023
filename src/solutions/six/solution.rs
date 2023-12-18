use crate::{error::SolutionError, util::SolutionPart};

struct DataPayload {
    races: Vec<(i64, i64)>,
}

fn parse_input_data(
    problem_data: String,
    solution_part: SolutionPart,
) -> Result<DataPayload, SolutionError> {
    let mut races = vec![];

    let (time_str, dist_str) = problem_data.split_once('\n').unwrap();
    let time_strs: Vec<&str> = time_str
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .filter(|c| c.len() > 0)
        .collect();
    let dist_strs: Vec<&str> = dist_str
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .filter(|c| c.len() > 0)
        .collect();

    let times: Vec<i64> = match solution_part {
        SolutionPart::PartOne => time_strs
            .iter()
            .map(|n_str| n_str.parse::<i64>().unwrap())
            .collect(),
        SolutionPart::PartTwo => vec![time_strs.join("").to_string().parse::<i64>().unwrap()],
    };

    let distances: Vec<i64> = match solution_part {
        SolutionPart::PartOne => dist_strs
            .iter()
            .map(|n_str| n_str.parse::<i64>().unwrap())
            .collect(),
        SolutionPart::PartTwo => vec![dist_strs.join("").to_string().parse::<i64>().unwrap()],
    };

    for i in 0..times.len() {
        races.push((
            times.get(i).unwrap().clone(),
            distances.get(i).unwrap().clone(),
        ));
    }

    Ok(DataPayload { races })
}

fn total_distance(time_held: i64, total_time: i64) -> i64 {
    time_held * (total_time - time_held)
}

fn solve(payload: &DataPayload) -> i64 {
    let mut options_product = 1;
    for race in &payload.races {
        let time = race.0;
        let distance = race.1;

        let mut time_to_beat = 0;
        while total_distance(time_to_beat, time) < distance {
            time_to_beat += 1;
        }
        options_product *= time - ((2 * time_to_beat) - 1);
    }
    options_product
}

pub fn run(problem_data: String, solution_part: SolutionPart) -> Result<String, SolutionError> {
    let payload = parse_input_data(problem_data, solution_part)?;

    Ok(solve(&payload).to_string())
}
