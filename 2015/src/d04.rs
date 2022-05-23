struct Solution {
	first_coin_integer: u64,
}

fn get_hash(input: &str, suffix_int: u64) -> String {
	let digest = md5::compute(format!("{}{}", input, suffix_int));
	format!("{:x}", digest)
}

fn solve(coin_goal: &str, input: &str) -> Solution {
	let mut suffix_int = 0;
	let mut hash = get_hash(input, suffix_int);

	while !hash.starts_with(coin_goal) {
		suffix_int += 1;
		hash = get_hash(input, suffix_int);
	}

	Solution {
		first_coin_integer: suffix_int,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	#[ignore] // These tests are expensive, f-ing proof of work
	fn part_1_example_cases() {
		let example1 = "abcdef";
		assert_eq!(solve("00000", example1).first_coin_integer, 609043);

		let example2 = "pqrstuv";
		assert_eq!(solve("00000", example2).first_coin_integer, 1048970);
	}

	#[test]
	#[ignore]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/04.txt").unwrap();

		assert_eq!(solve("00000", input.trim()).first_coin_integer, 346386);
	}

	#[test]
	#[ignore]
	fn part_2_example_cases() {
		// No examples
	}

	#[test]
	#[ignore]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/04.txt").unwrap();

		assert_eq!(solve("000000", input.trim()).first_coin_integer, 9958218);
	}
}
