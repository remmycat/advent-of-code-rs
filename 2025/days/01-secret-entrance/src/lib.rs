use aoc_utils::{ascii_int::parse_uint, trim::trim_end_newline};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

const NEWLINE: u8 = b'\n';

pub fn solve(input: &[u8]) -> Solution {
	let input = trim_end_newline(input);

	let mut rot_current = 50_usize;
	let mut rot_to_zero = 0_usize;
	let mut zero_passed = 0_usize;

	for line in input.split(|b| *b == NEWLINE) {
		let ticks = parse_uint(&line[1..]);
		let lefty = line[0] == b'L';

		if lefty {
			// translate into a mirror world
			rot_current = (100 - rot_current) % 100
		}

		let rot_target = rot_current + ticks;
		zero_passed += rot_target / 100;
		rot_current = rot_target % 100;

		if lefty {
			// translate back
			rot_current = (100 - rot_current) % 100
		}

		if rot_current == 0 {
			rot_to_zero += 1;
		}
	}

	Solution(rot_to_zero, zero_passed)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(3,6),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(1141,6634),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
