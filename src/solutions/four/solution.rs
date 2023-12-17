use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use crate::{error::SolutionError, util::SolutionPart};

struct ScratchCard {
    winning_numbers: HashSet<u32>,
    contained_numbers: Vec<u32>,
    num_winning_numbers: u32,
    points: u32,
}

impl From<&str> for ScratchCard {
    fn from(value: &str) -> Self {
        let mut winning_numbers: HashSet<u32> = HashSet::new();
        let mut contained_numbers: Vec<u32> = vec![];

        let card_numbers = value.split_once(':').unwrap().1.split_once('|').unwrap();

        for winning_number in card_numbers.0.trim().split(' ') {
            if let Ok(num) = winning_number.trim().parse::<u32>() {
                winning_numbers.insert(num);
            }
        }
        for contained_number in card_numbers.1.trim().split(' ') {
            if let Ok(num) = contained_number.trim().parse::<u32>() {
                contained_numbers.push(num)
            }
        }

        let num_winning_numbers = contained_numbers
            .iter()
            .filter(|cn| winning_numbers.contains(cn))
            .count() as u32;

        let points: u32 = match num_winning_numbers {
            0 => 0,
            _ => 2_u32.pow(num_winning_numbers as u32 - 1),
        };

        ScratchCard {
            winning_numbers,
            contained_numbers,
            num_winning_numbers,
            points,
        }
    }
}

impl Display for ScratchCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let winning_num_strs: Vec<String> = self
            .winning_numbers
            .iter()
            .map(|num| num.to_string())
            .collect();
        let contained_num_strs: Vec<String> = self
            .contained_numbers
            .iter()
            .map(|num| num.to_string())
            .collect();

        write!(
            f,
            "Winning: {}\nContained: {}",
            winning_num_strs.join(","),
            contained_num_strs.join(",")
        )
    }
}

pub fn run(problem_data: String, solution_part: SolutionPart) -> Result<String, SolutionError> {
    let mut cards: Vec<ScratchCard> = vec![];
    for line in problem_data.split('\n') {
        let card = ScratchCard::from(line);
        cards.push(card);
    }

    match solution_part {
        SolutionPart::PartOne => Ok(part_one_solution(cards).to_string()),
        SolutionPart::PartTwo => Ok(part_two_solution(cards).to_string()),
    }
}

fn part_two_solution(cards: Vec<ScratchCard>) -> i32 {
    let card_map: HashMap<usize, &ScratchCard> = cards
        .iter()
        .enumerate()
        .map(|entry| (entry.0, entry.1))
        .collect();
    let mut card_deque: VecDeque<(usize, &ScratchCard)> = cards.iter().enumerate().collect();
    let mut answer = 0;
    while let Some((index, card)) = card_deque.pop_front() {
        answer += 1;
        for i in 1..card.num_winning_numbers + 1 {
            let new_card_index = index + i as usize;
            card_deque.push_back((new_card_index, card_map[&new_card_index]))
        }
    }
    answer
}

fn part_one_solution(cards: Vec<ScratchCard>) -> i32 {
    cards.iter().map(|card| card.points).sum::<u32>() as i32
}
