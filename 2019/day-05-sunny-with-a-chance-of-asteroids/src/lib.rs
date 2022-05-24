use std::str::FromStr;

use intcode::{IntCodeError, IntCodeProgram};

pub struct Solution {
	pub ac_diagnostic: isize,
	pub tr_diagnostic: isize,
}

const AC_UNIT_ID: isize = 1;
const THERMAL_RADIATOR_ID: isize = 5;

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let mut program = IntCodeProgram::from_str(input)?;

	let outputs_ac = program.run(&[AC_UNIT_ID])?;

	for (i, output) in outputs_ac.iter().enumerate() {
		if i < outputs_ac.len() - 1 {
			assert_eq!(*output, 0);
		}
	}

	let ac_diagnostic = *outputs_ac
		.last()
		.expect("expected to have at least one output");

	program.reset();

	let outputs_tr = program.run(&[THERMAL_RADIATOR_ID])?;

	for (i, output) in outputs_tr.iter().enumerate() {
		if i < outputs_tr.len() - 1 {
			assert_eq!(*output, 0);
		}
	}

	let tr_diagnostic = *outputs_tr
		.last()
		.expect("expected to have at least one output");

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
