pub struct Solution {
	pub max_calories: u64,
	pub max_3_calories: u64,
}

pub fn solve_iterators(input: &str) -> Solution {
	let mut elves: Vec<u64> = input
		.trim()
		.split("\n\n")
		.map(|block| {
			block
				.split('\n')
				.map(|line| {
					line.parse::<u64>()
						.expect("expected input lines to be parsable as u64")
				})
				.sum()
		})
		.collect();

	elves.sort_unstable();

	let max_calories = elves.iter().rev().take(1).sum();
	let max_3_calories = elves.iter().rev().take(3).sum();

	Solution {
		max_calories,
		max_3_calories,
	}
}

pub fn solve_loop(input: &str) -> Solution {
	let mut elves: Vec<u64> = vec![];
	let mut reading_elf = false;

	for line in input.lines() {
		if line.is_empty() {
			reading_elf = false;
			continue;
		}

		let calories: u64 = line.parse().expect("input line not parsable as int");

		if !reading_elf {
			elves.push(calories);
			reading_elf = true;
			continue;
		}

		let elf = elves.last_mut().expect("impossible state");

		*elf += calories;
	}

	elves.sort_unstable();

	let max_calories: u64 = elves.iter().rev().take(1).sum();
	let max_3_calories: u64 = elves.iter().rev().take(3).sum();

	Solution {
		max_calories,
		max_3_calories,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::include_str;

	const EXAMPLE: &str = include_str!("../inputs/example.txt");
	const PERSONAL: &str = include_str!("../inputs/personal.txt");

	// PART 1

	#[test]
	fn part_1_example_iter() {
		assert_eq!(solve_iterators(EXAMPLE).max_calories, 24000);
	}

	#[test]
	fn part_1_example_loop() {
		assert_eq!(solve_loop(EXAMPLE).max_calories, 24000);
	}

	#[test]
	fn part_1_personal_iter() {
		assert_eq!(solve_iterators(PERSONAL).max_calories, 74198);
	}

	#[test]
	fn part_1_personal_loop() {
		assert_eq!(solve_loop(PERSONAL).max_calories, 74198);
	}

	// PART 2

	#[test]
	fn part_2_example_iter() {
		assert_eq!(solve_iterators(EXAMPLE).max_3_calories, 45000);
	}

	#[test]
	fn part_2_example_loop() {
		assert_eq!(solve_loop(EXAMPLE).max_3_calories, 45000);
	}

	#[test]
	fn part_2_personal_iter() {
		assert_eq!(solve_iterators(PERSONAL).max_3_calories, 209914);
	}

	#[test]
	fn part_2_personal_loop() {
		assert_eq!(solve_loop(PERSONAL).max_3_calories, 209914);
	}
}
