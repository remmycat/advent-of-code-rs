use std::str::FromStr;

use intcode::{operation::ParameterMode, IntCodeError, IntCodeProgram};

#[derive(Debug, Clone, Copy)]
pub struct Params {
	noun: isize,
	verb: isize,
}

pub struct Solution {
	pub output_1202: isize,
	pub secret_params: Params,
}

const PARAMS_1202: Params = Params { noun: 12, verb: 2 };

const REVERSE_RESULT: isize = 19690720;

fn run_program(program: &mut IntCodeProgram, params: Params) -> Result<isize, IntCodeError> {
	program.reset();

	program.write(1, params.noun)?;
	program.write(2, params.verb)?;
	program.run(&[])?;

	program.read(0, ParameterMode::Immediate)?.value()
}

fn backsolve(program: &mut IntCodeProgram, wanted_result: isize) -> Result<Params, IntCodeError> {
	for noun in 0..=99 {
		for verb in 0..=99 {
			let params = Params { noun, verb };
			let result = run_program(program, params)?;
			if result == wanted_result {
				return Ok(params);
			}
		}
	}

	panic!("Result impossible")
}

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let mut program = IntCodeProgram::from_str(input)?;

	Ok(Solution {
		output_1202: run_program(&mut program, PARAMS_1202)?,
		secret_params: backsolve(&mut program, REVERSE_RESULT)?,
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
