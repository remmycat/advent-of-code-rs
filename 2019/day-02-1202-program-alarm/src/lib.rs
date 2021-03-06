use std::str::FromStr;

use fallible_iterator::{FallibleIterator, IntoFallibleIterator};
use intcode::{IntCodeError, IntCodeProgram, IntCodeProgramIter};

#[derive(Debug, Clone, Copy)]
pub struct Params {
	noun: i64,
	verb: i64,
}

pub struct Solution {
	pub output_1202: i64,
	pub secret_params: Params,
}

const PARAMS_1202: Params = Params { noun: 12, verb: 2 };

const REVERSE_RESULT: i64 = 19690720;

fn run_program(program: IntCodeProgram, params: Params) -> Result<i64, IntCodeError> {
	let mut patched: IntCodeProgramIter = program
		.patch(1, params.noun)?
		.patch(2, params.verb)?
		.into_fallible_iter();

	assert_eq!(patched.next()?, None, "This program does not have outputs");

	Ok(patched.read_value(0))
}

fn backsolve(program: IntCodeProgram, wanted_result: i64) -> Result<Params, IntCodeError> {
	for noun in 0..=99 {
		for verb in 0..=99 {
			let params = Params { noun, verb };
			let result = run_program(program.clone(), params)?;
			if result == wanted_result {
				return Ok(params);
			}
		}
	}

	panic!("Result impossible")
}

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let program = IntCodeProgram::from_str(input)?;

	Ok(Solution {
		output_1202: run_program(program.clone(), PARAMS_1202)?,
		secret_params: backsolve(program, REVERSE_RESULT)?,
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

		assert_eq!(solve(&input)?.output_1202, 4945026);
		Ok(())
	}

	#[test]
	fn part_2_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		let params = solve(&input)?.secret_params;

		assert_eq!(params.noun, 52);
		assert_eq!(params.verb, 96);
		assert_eq!(params.noun * 100 + params.verb, 5296);
		Ok(())
	}
}
