use std::{collections::VecDeque, str::FromStr};

use anyhow::Error;

struct Solution {
	fish_count_80: u128,
	fish_count_256: u128,
}

struct Population {
	by_inv: VecDeque<u128>,
}

impl Population {
	fn new() -> Self {
		Population {
			by_inv: VecDeque::from([0; 9]),
		}
	}

	fn day_cycle(&mut self) {
		let spawning = self.by_inv[0];

		self.by_inv.rotate_left(1);

		self.by_inv[6] += spawning;
	}

	fn count_fishies(&self) -> u128 {
		self.by_inv.iter().sum()
	}
}

impl FromStr for Population {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut initial_fishes = Population::new();

		for inv_age in s.split(',').map(|f| f.parse::<usize>().unwrap()) {
			initial_fishes.by_inv[inv_age] += 1;
		}

		Ok(initial_fishes)
	}
}

fn solve(input: &str) -> Solution {
	let mut pop = input.parse::<Population>().unwrap();

	for _day in 1..=80 {
		pop.day_cycle();
	}

	let fish_count_80 = pop.count_fishies();

	let rest_days = 256 - 80;

	for _day in 1..=rest_days {
		pop.day_cycle();
	}

	let fish_count_256 = pop.count_fishies();

	Solution {
		fish_count_80,
		fish_count_256,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("3,4,3,1,2").fish_count_80, 5934);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_06.txt").unwrap();

		assert_eq!(solve(input.trim()).fish_count_80, 360268);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve("3,4,3,1,2").fish_count_256, 26984457539);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_06.txt").unwrap();

		assert_eq!(solve(input.trim()).fish_count_256, 1632146183902);
	}
}
