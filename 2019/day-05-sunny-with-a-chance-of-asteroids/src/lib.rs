use std::str::FromStr;

use intcode::{IntCodeError, IntCodeProgram};

pub struct Solution {
	pub ac_diagnostic: isize,
	pub tr_diagnostic: isize,
}

const AC_UNIT_ID: isize = 1;
const THERMAL_RADIATOR_ID: isize = 5;

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let program = IntCodeProgram::from_str(input)?;

	let mut ac_outputs = program.clone().inputs(vec![AC_UNIT_ID]).run()?;

	let ac_diagnostic = ac_outputs
		.pop()
		.expect("expected to have at least one output");

	for other_output in ac_outputs {
		assert_eq!(other_output, 0);
	}

	let mut tr_outputs = program.inputs(vec![THERMAL_RADIATOR_ID]).run()?;

	let tr_diagnostic = tr_outputs
		.pop()
		.expect("expected to have at least one output");

	for other_output in tr_outputs {
		assert_eq!(other_output, 0);
	}

	Ok(Solution {
		ac_diagnostic,
		tr_diagnostic,
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

		assert_eq!(solve(&input)?.ac_diagnostic, 13285749);
		Ok(())
	}

	#[test]
	fn part_2_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input)?.tr_diagnostic, 5000972);
		Ok(())
	}
}
