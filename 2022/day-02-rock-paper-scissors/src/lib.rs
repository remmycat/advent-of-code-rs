pub struct Solution {
	pub score_if_first_read_correct: u64,
	pub actual_score: u64,
}

const THEIR_ROCK: char = 'A';
const THEIR_PAPER: char = 'B';
const THEIR_SCISSORS: char = 'C';

const MY_ROCK: char = 'X';
const MY_PAPER: char = 'Y';
const MY_SCISSORS: char = 'Z';

const NEED_LOSE: char = 'X';
const NEED_DRAW: char = 'Y';
const NEED_WIN: char = 'Z';

pub fn solve(input: &str) -> Solution {
	let mut wrong_score: u64 = 0;
	let mut actual_score: u64 = 0;

	for line in input.trim().lines() {
		let mut chars = line.chars();
		let theirs = chars.next().expect("malformed input");
		let mine = chars.nth(1).expect("malformed input");

		debug_assert!(chars.next().is_none());

		let wrong_choice_score: u64 = match mine {
			MY_ROCK => 1,
			MY_PAPER => 2,
			MY_SCISSORS => 3,
			_ => panic!("malformed input"),
		};

		let wrong_result_score: u64 = match (mine, theirs) {
			(MY_ROCK, THEIR_PAPER) | (MY_PAPER, THEIR_SCISSORS) | (MY_SCISSORS, THEIR_ROCK) => 0,
			(MY_ROCK, THEIR_ROCK) | (MY_PAPER, THEIR_PAPER) | (MY_SCISSORS, THEIR_SCISSORS) => 3,
			(MY_ROCK, THEIR_SCISSORS) | (MY_PAPER, THEIR_ROCK) | (MY_SCISSORS, THEIR_PAPER) => 6,
			_ => panic!("malformed input"),
		};

		wrong_score += wrong_choice_score + wrong_result_score;

		let result_score: u64 = match mine {
			NEED_LOSE => 0,
			NEED_DRAW => 3,
			NEED_WIN => 6,
			_ => panic!("malformed input"),
		};

		let choice_score: u64 = match (mine, theirs) {
			(NEED_LOSE, THEIR_PAPER) | (NEED_DRAW, THEIR_ROCK) | (NEED_WIN, THEIR_SCISSORS) => 1,
			(NEED_LOSE, THEIR_SCISSORS) | (NEED_DRAW, THEIR_PAPER) | (NEED_WIN, THEIR_ROCK) => 2,
			(NEED_LOSE, THEIR_ROCK) | (NEED_DRAW, THEIR_SCISSORS) | (NEED_WIN, THEIR_PAPER) => 3,
			_ => panic!("malformed input"),
		};

		actual_score += result_score + choice_score;
	}

	Solution {
		score_if_first_read_correct: wrong_score,
		actual_score,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example =
			fs::read_to_string("inputs/example.txt").expect("Error reading example input file");

		assert_eq!(solve(&example).score_if_first_read_correct, 15);
	}

	#[test]
	fn part_1_solution() {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).score_if_first_read_correct, 10816);
	}

	#[test]
	fn part_2_example_cases() {
		let example =
			fs::read_to_string("inputs/example.txt").expect("Error reading example input file");

		assert_eq!(solve(&example).actual_score, 12);
	}

	#[test]
	fn part_2_solution() {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).actual_score, 11657);
	}
}
