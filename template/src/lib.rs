pub struct Solution {
	pub value1: u64,
	pub value2: u64,
}

pub fn solve(input: &str) -> Solution {
	Solution {
		value1: 0,
		value2: 0,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::include_str;

	const EXAMPLE: &str = include_str!("../inputs/example.txt");
	const PERSONAL: &str = include_str!("../inputs/personal.txt");

	#[test]
	fn part_1_example() {
		assert_eq!(solve(EXAMPLE).value1, todo!());
	}

	#[test]
	fn part_1_personal() {
		assert_eq!(solve(PERSONAL).value1, todo!());
	}

	#[test]
	fn part_2_example() {
		assert_eq!(solve(EXAMPLE).value2, todo!());
	}

	#[test]
	fn part_2_personal() {
		assert_eq!(solve(PERSONAL).value2, todo!());
	}
}
