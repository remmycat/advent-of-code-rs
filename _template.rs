pub struct Solution {
	pub value: u64,
}

pub fn solve(input: &str) -> Solution {
	Solution {
		value: unimplemented!(),
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

		assert_eq!(solve(&example).value, 0);
	}

	#[test]
	fn part_1_solution() {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).value, 0);
	}

	#[test]
	fn part_2_example_cases() {
		unimplemented!();
	}

	#[test]
	fn part_2_solution() {
		unimplemented!();
	}
}
