pub struct Solution {
	pub max_calories: u64,
	pub max_3_calories: u64,
}

pub fn solve(input: &str) -> Solution {
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

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example =
			fs::read_to_string("inputs/example.txt").expect("Error reading example input file");

		assert_eq!(solve(&example).max_calories, 24000);
	}

	#[test]
	fn part_1_solution() {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).max_calories, 74198);
	}

	#[test]
	fn part_2_example_cases() {
		let example =
			fs::read_to_string("inputs/example.txt").expect("Error reading example input file");

		assert_eq!(solve(&example).max_3_calories, 45000);
	}

	#[test]
	fn part_2_solution() {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).max_3_calories, 209914);
	}
}
