#[derive(Debug, PartialEq, Eq)]
pub struct Solution(u64, u64);

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

	let max_energy = elves.iter().rev().take(1).sum();
	let max_3_energies = elves.iter().rev().take(3).sum();

	Solution(max_energy, max_3_energies)
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

	Solution(max_calories, max_3_calories)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;
	use std::include_str;

	#[rstest]
	#[case(include_str!("../inputs/example.txt"), Solution(24000, 45000))]
	#[case(include_str!("../inputs/personal.txt"), Solution(74198,209914))]
	fn solution(
		#[case] input: &str,
		#[case] expected: Solution,
		#[values(solve_iterators, solve_loop)] solver: impl Fn(&str) -> Solution,
	) {
		assert_eq!(solver(input), expected);
	}
}
