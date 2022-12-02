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

enum GameResult {
	Loss,
	Draw,
	Win,
}

impl GameResult {
	const fn score(&self) -> u64 {
		match self {
			GameResult::Loss => 0,
			GameResult::Draw => 3,
			GameResult::Win => 6,
		}
	}
}

#[derive(Clone, Copy)]
enum Shape {
	Rock,
	Paper,
	Scissors,
}

impl Shape {
	const fn score(&self) -> u64 {
		use Shape::{Paper, Rock, Scissors};
		match self {
			Rock => 1,
			Paper => 2,
			Scissors => 3,
		}
	}

	const fn get_result(self, opposing_shape: Shape) -> GameResult {
		use GameResult::{Draw, Loss, Win};
		use Shape::{Paper, Rock, Scissors};
		match self {
			Rock => match opposing_shape {
				Rock => Draw,
				Paper => Loss,
				Scissors => Win,
			},
			Paper => match opposing_shape {
				Rock => Win,
				Paper => Draw,
				Scissors => Loss,
			},
			Scissors => match opposing_shape {
				Rock => Loss,
				Paper => Win,
				Scissors => Draw,
			},
		}
	}

	const fn find_response_for_outcome(self, outcome: GameResult) -> Self {
		use GameResult::{Draw, Loss, Win};
		use Shape::{Paper, Rock, Scissors};
		match outcome {
			Draw => self,
			Loss => match self {
				Rock => Scissors,
				Paper => Rock,
				Scissors => Paper,
			},
			Win => match self {
				Rock => Paper,
				Paper => Scissors,
				Scissors => Rock,
			},
		}
	}
}

const fn get_wrong_score(theirs: Shape, ours: Shape) -> u64 {
	ours.score() + ours.get_result(theirs).score()
}

const fn get_correct_score(theirs: Shape, outcome: GameResult) -> u64 {
	outcome.score() + theirs.find_response_for_outcome(outcome).score()
}

const fn get_both_scores(theirs: Shape, ours: Shape, or_result: GameResult) -> (u64, u64) {
	(
		get_wrong_score(theirs, ours),
		get_correct_score(theirs, or_result),
	)
}

fn get_round_scores(line: &[u8]) -> (u64, u64) {
	use GameResult::{Draw, Loss, Win};
	use Shape::{Paper, Rock, Scissors};

	#[allow(clippy::identity_op)]
	match line {
		ROCK_X => get_both_scores(Rock, Rock, Loss),
		ROCK_Y => get_both_scores(Rock, Paper, Draw),
		ROCK_Z => get_both_scores(Rock, Scissors, Win),
		PAPER_X => get_both_scores(Paper, Rock, Loss),
		PAPER_Y => get_both_scores(Paper, Paper, Draw),
		PAPER_Z => get_both_scores(Paper, Scissors, Win),
		SCISSORS_X => get_both_scores(Scissors, Rock, Loss),
		SCISSORS_Y => get_both_scores(Scissors, Paper, Draw),
		SCISSORS_Z => get_both_scores(Scissors, Scissors, Win),
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
