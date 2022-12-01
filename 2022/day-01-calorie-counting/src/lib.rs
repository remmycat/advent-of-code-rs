pub struct Solution {
	pub max_calories: u64,
	pub max_3_calories: u64,
}

pub fn solve(input: &str) -> Solution {
	let mut elves: Vec<u64> = vec![];

	let mut elve: Vec<u64> = vec![];

	for line in input.lines() {
		if line.is_empty() {
			elves.push(elve.into_iter().sum());
			elve = vec![];
		} else {
			elve.push(line.parse().expect("input line not parsable as int"))
		}
	}

	if !elve.is_empty() {
		elves.push(elve.into_iter().sum());
	}

	elves.sort_unstable();
	elves.reverse();

	let max_calories = elves.iter().take(1).sum();
	let max_3_calories = elves.iter().take(3).sum();

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
