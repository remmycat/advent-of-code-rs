use std::str::FromStr;

use intcode::{IntCodeError, IntCodeProgram};

pub struct Solution {
	pub boost_keycode: i64,
	pub distress_coordinates: i64,
}

const TEST_MODE_INPUT: i64 = 1;
const BOOST_MODE_INPUT: i64 = 2;

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let program = IntCodeProgram::from_str(input)?;

	let test_outputs = program.clone().inputs(vec![TEST_MODE_INPUT]).run()?;
	assert_eq!(
		test_outputs.len(),
		1,
		"Test program is said to have only 1 output"
	);

	let boost_keycode = test_outputs[0];

	let boost_putputs = program.inputs(vec![BOOST_MODE_INPUT]).run()?;
	assert_eq!(
		boost_putputs.len(),
		1,
		"Boost program is said to have only 1 output"
	);

	let distress_coordinates = boost_putputs[0];

	Ok(Solution {
		boost_keycode,
		distress_coordinates,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input)?.boost_keycode, 4261108180);
		Ok(())
	}

	#[test]
	fn part_2_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input)?.distress_coordinates, 77944);
		Ok(())
	}
}
