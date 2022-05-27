use std::str::FromStr;

use intcode::{IntCodeError, IntCodeProgram};

pub struct Solution {
	pub ac_diagnostic: i64,
	pub tr_diagnostic: i64,
}

const AC_UNIT_ID: i64 = 1;
const THERMAL_RADIATOR_ID: i64 = 5;

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let program = IntCodeProgram::from_str(input)?;

	let mut ac_outputs = program.clone().inputs(vec![AC_UNIT_ID]).run()?;

	let ac_diagnostic = ac_outputs
		.pop()
		.expect("AC Program is expected to have at least one output");

	for other_output in ac_outputs {
		assert_eq!(other_output, 0, "All outputs except for the last one have to be 0, or there is an error in the intcode handling");
	}

	let mut tr_outputs = program.inputs(vec![THERMAL_RADIATOR_ID]).run()?;

	let tr_diagnostic = tr_outputs
		.pop()
		.expect("Thermal Radiator program is expected to have at least one output");

	for other_output in tr_outputs {
		assert_eq!(other_output, 0, "All outputs except for the last one have to be 0, or there is an error in the intcode handling");
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
