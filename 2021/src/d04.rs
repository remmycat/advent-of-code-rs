use anyhow::{Error, Result};
use std::str::FromStr;

pub struct Solution {
	first_win_score: u128,
	last_win_score: u128,
}

const BOARD_SIZE: usize = 5;

struct BingoCard {
	// row[column[]]
	rows: [[u128; BOARD_SIZE]; BOARD_SIZE],
	cols: [[u128; BOARD_SIZE]; BOARD_SIZE],
	called: Vec<u128>,
	won: bool,
}

impl FromStr for BingoCard {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		// Iter.collect().as_slice().try_into() does an implicit check that the data
		// has the expected size (5). Neat!
		let rows: [[u128; BOARD_SIZE]; BOARD_SIZE] = s
			.lines()
			.map(|line| -> [u128; BOARD_SIZE] {
				line.split_whitespace()
					.map(|num| num.trim().parse::<u128>().unwrap())
					.collect::<Vec<_>>()
					.as_slice()
					.try_into()
					.unwrap()
			})
			.collect::<Vec<_>>()
			.as_slice()
			.try_into()
			.unwrap();

		let mut cols = [[0_u128; BOARD_SIZE]; BOARD_SIZE];

		for (i, row) in rows.iter().enumerate() {
			for (j, num) in row.iter().enumerate() {
				cols[j][i] = *num;
			}
		}

		Ok(BingoCard {
			rows,
			cols,
			called: vec![],
			won: false,
		})
	}
}

impl BingoCard {
	fn call(self, num: u128) -> Self {
		let mut called = self.called;
		called.push(num);

		let mut won = false;

		self.rows.iter().for_each(|row| {
			if row.iter().all(|num| called.contains(num)) {
				won = true;
			}
		});

		self.cols.iter().for_each(|col| {
			if col.iter().all(|num| called.contains(num)) {
				won = true;
			}
		});

		BingoCard {
			cols: self.cols,
			rows: self.rows,
			won,
			called,
		}
	}

	fn score(&self) -> Option<u128> {
		let mut unmarked_sum = 0;

		self.rows.iter().for_each(|row| {
			row.iter().for_each(|num| {
				if !self.called.contains(num) {
					unmarked_sum += num
				}
			})
		});

		self.called.last().map(|last| last * unmarked_sum)
	}
}

pub fn solve(input: &str) -> Solution {
	let mut blocks = input.split("\n\n");

	let called: Vec<_> = blocks
		.next()
		.unwrap()
		.split(',')
		.map(|num| num.parse::<u128>().unwrap())
		.collect();

	let mut cards: Vec<BingoCard> = blocks.map(|card| card.parse().unwrap()).collect();
	let initial_cards = cards.len();

	let mut first_win_score = 0;
	let mut last_win_score = 0;

	for num in called {
		cards = cards.into_iter().map(|c| c.call(num)).collect();

		let not_won: &Vec<u128> = &cards
			.iter()
			.filter_map(|c| if !c.won { c.score() } else { None })
			.collect();

		let won: &Vec<u128> = &cards
			.iter()
			.filter_map(|c| if c.won { c.score() } else { None })
			.collect();

		if cards.len() == initial_cards && !won.is_empty() {
			first_win_score = *(won.get(0).unwrap());
		}

		if not_won.is_empty() && won.len() == 1 {
			last_win_score = *(won.get(0).unwrap());
			break;
		}

		cards = cards.into_iter().filter(|c| !c.won).collect();
	}

	Solution {
		first_win_score,
		last_win_score,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

		let solution = solve(example);

		assert_eq!(solution.first_win_score, 4512);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/04.txt").unwrap();

		let solution = solve(&input);

		assert_eq!(solution.first_win_score, 28082);
	}

	#[test]
	fn part_2_example_cases() {
		let example = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
		let solution = solve(example);

		assert_eq!(solution.last_win_score, 1924);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/04.txt").unwrap();

		let solution = solve(&input);

		assert_eq!(solution.last_win_score, 8224);
	}
}
