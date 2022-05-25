use fallible_iterator::{FallibleIterator, IntoFallibleIterator};
use intcode::{IntCodeError, IntCodeProgram};
use itertools::Itertools;
use std::str::FromStr;

pub struct Solution {
	pub max_thruster_signal: isize,
	pub max_looped_thruster_signal: isize,
}

fn get_thruster_signal(
	program: &IntCodeProgram,
	phase_sequence: Vec<isize>,
	loopdiloop: bool,
) -> Result<isize, IntCodeError> {
	let mut signal = 0;
	let mut programs = vec![];

	for phase in phase_sequence {
		let mut iter = program
			.clone()
			.inputs(vec![phase, signal])
			.into_fallible_iter();

		signal = iter
			.next()?
			.expect("Each amp is said to have at least one output");

		programs.push(iter);
	}

	if loopdiloop {
		for i in (0..programs.len()).cycle() {
			programs[i].add_input(signal);

			if let Some(out) = programs[i].next()? {
				signal = out;
			} else {
				assert_eq!(
					i, 0,
					"The first amp is said to be the first one that has its program halting"
				);
				break;
			}
		}
	}

	for mut program in programs {
		assert_eq!(
			program.next()?,
			None,
			"All amps should have halted at this point"
		)
	}

	Ok(signal)
}

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let program = IntCodeProgram::from_str(input)?;
	let mut max_signal = 0;

	for phase_sequence in (0_isize..=4).permutations(5) {
		let signal = get_thruster_signal(&program, phase_sequence, false)?;
		if signal > max_signal {
			max_signal = signal;
		}
	}

	let mut max_looped_signal = 0;
	for phase_sequence in (5_isize..=9).permutations(5) {
		let signal = get_thruster_signal(&program, phase_sequence, true)?;
		if signal > max_looped_signal {
			max_looped_signal = signal;
		}
	}

	Ok(Solution {
		max_thruster_signal: max_signal,
		max_looped_thruster_signal: max_looped_signal,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_examples() -> Result<(), IntCodeError> {
		let code1 = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
		assert_eq!(solve(code1)?.max_thruster_signal, 43210);

		let code2 = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
		assert_eq!(solve(code2)?.max_thruster_signal, 54321);

		let code3 = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
		assert_eq!(solve(code3)?.max_thruster_signal, 65210);

		Ok(())
	}

	#[test]
	fn part_1_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input)?.max_thruster_signal, 21760);
		Ok(())
	}

	#[test]
	fn part_2_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input)?.max_looped_thruster_signal, 69816958);
		Ok(())
	}
}
