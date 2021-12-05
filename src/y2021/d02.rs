use anyhow::{bail, Error, Result};
use std::str::FromStr;

struct Solution {
	depth: i128,
	distance: i128,
	aimed_depth: i128,
}

enum Move {
	Forward(i128),
	Down(i128),
	Up(i128),
}

impl FromStr for Move {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let v: Vec<_> = s.split(' ').collect();

		match &v[..] {
			["forward", amt] => Ok(Move::Forward(amt.parse()?)),
			["down", amt] => Ok(Move::Down(amt.parse()?)),
			["up", amt] => Ok(Move::Up(amt.parse()?)),
			_ => bail!("Bad input"),
		}
	}
}

#[derive(Debug)]
struct Submarine {
	depth: i128,
	distance: i128,
	aim: i128,
	aimed_depth: i128,
}

impl Submarine {
	fn new() -> Self {
		Submarine {
			depth: 0,
			distance: 0,
			aim: 0,
			aimed_depth: 0,
		}
	}

	fn drive(&mut self, m: Move) {
		match m {
			Move::Forward(x) => {
				self.distance += x;
				self.aimed_depth += x * self.aim;
			}
			Move::Up(x) => {
				self.depth -= x;
				self.aim -= x;
			}
			Move::Down(x) => {
				self.depth += x;
				self.aim += x;
			}
		};
	}
}

fn solve(input: &str) -> Solution {
	let mut sub = Submarine::new();

	input
		.lines()
		.map(|l| l.parse::<Move>().unwrap())
		.for_each(|m| sub.drive(m));

	Solution {
		depth: sub.depth,
		distance: sub.distance,
		aimed_depth: sub.aimed_depth,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
		let solution = solve(example);

		assert_eq!(solution.distance, 15);
		assert_eq!(solution.depth, 10);
		assert_eq!(solution.distance * solution.depth, 150);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_02.txt").unwrap();

		let solution = solve(&input);

		assert_eq!(solution.distance, 2083);
		assert_eq!(solution.depth, 955);
		assert_eq!(solution.distance * solution.depth, 1989265);
	}

	#[test]
	fn part_2_example_cases() {
		let example = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
		let solution = solve(example);

		assert_eq!(solution.distance, 15);
		assert_eq!(solution.aimed_depth, 60);
		assert_eq!(solution.distance * solution.aimed_depth, 900);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_02.txt").unwrap();

		let solution = solve(&input);

		assert_eq!(solution.distance, 2083);
		assert_eq!(solution.aimed_depth, 1002964);
		assert_eq!(solution.distance * solution.aimed_depth, 2089174012);
	}
}
