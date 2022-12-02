pub struct Solution {
	pub score_if_first_read_correct: u64,
	pub actual_score: u64,
}

const ROCK_X: &[u8] = b"A X\n";
const ROCK_Y: &[u8] = b"A Y\n";
const ROCK_Z: &[u8] = b"A Z\n";
const PAPER_X: &[u8] = b"B X\n";
const PAPER_Y: &[u8] = b"B Y\n";
const PAPER_Z: &[u8] = b"B Z\n";
const SCISSORS_X: &[u8] = b"C X\n";
const SCISSORS_Y: &[u8] = b"C Y\n";
const SCISSORS_Z: &[u8] = b"C Z\n";

mod scores {
	pub const ROCK: u64 = 1;
	pub const PAPER: u64 = 2;
	pub const SCISSORS: u64 = 3;

	pub const LOSS: u64 = 0;
	pub const DRAW: u64 = 3;
	pub const WIN: u64 = 6;

	pub const ROCK_ROCK: u64 = ROCK + DRAW;
	pub const ROCK_PAPER: u64 = PAPER + WIN;
	pub const ROCK_SCISSORS: u64 = SCISSORS + LOSS;
	pub const PAPER_ROCK: u64 = ROCK + LOSS;
	pub const PAPER_PAPER: u64 = PAPER + DRAW;
	pub const PAPER_SCISSORS: u64 = SCISSORS + WIN;
	pub const SCISSORS_ROCK: u64 = ROCK + WIN;
	pub const SCISSORS_PAPER: u64 = PAPER + LOSS;
	pub const SCISSORS_SCISSORS: u64 = SCISSORS + DRAW;
}

fn get_round_scores(line: &[u8]) -> (u64, u64) {
	use scores::*;

	#[allow(clippy::identity_op)]
	match line {
		ROCK_X => (ROCK_ROCK, ROCK_SCISSORS),
		ROCK_Y => (ROCK_PAPER, ROCK_ROCK),
		ROCK_Z => (ROCK_SCISSORS, ROCK_PAPER),
		PAPER_X => (PAPER_ROCK, PAPER_ROCK),
		PAPER_Y => (PAPER_PAPER, PAPER_PAPER),
		PAPER_Z => (PAPER_SCISSORS, PAPER_SCISSORS),
		SCISSORS_X => (SCISSORS_ROCK, SCISSORS_PAPER),
		SCISSORS_Y => (SCISSORS_PAPER, SCISSORS_SCISSORS),
		SCISSORS_Z => (SCISSORS_SCISSORS, SCISSORS_ROCK),
		[] => (0, 0),
		_ => panic!("malformed input"),
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let mut wrong_score: u64 = 0;
	let mut actual_score: u64 = 0;

	for (wrong, actual) in input.chunks_exact(4).map(get_round_scores) {
		wrong_score += wrong;
		actual_score += actual;
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
		let example = fs::read("inputs/example.txt").expect("Error reading example input file");

		assert_eq!(solve(&example).score_if_first_read_correct, 15);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).score_if_first_read_correct, 10816);
	}

	#[test]
	fn part_2_example_cases() {
		let example = fs::read("inputs/example.txt").expect("Error reading example input file");

		assert_eq!(solve(&example).actual_score, 12);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).actual_score, 11657);
	}
}
