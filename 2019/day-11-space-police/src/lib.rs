mod hull;
mod robot;

use hull::Color;
use intcode::{IntCodeError, IntCodeProgram};
use robot::Robot;
use std::str::FromStr;

pub struct Solution {
	pub first_try_painted_tiles: usize,
	pub registration_identifier: String,
}

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let program = IntCodeProgram::from_str(input)?;

	let first_try_hull_map = Robot::new().run_program(program.clone(), Color::Black)?;
	let second_try_hull_map = Robot::new().run_program(program, Color::White)?;

	Ok(Solution {
		first_try_painted_tiles: first_try_hull_map.painted_len(),
		registration_identifier: format!("{second_try_hull_map}"),
	})
}

#[cfg(test)]
mod tests {
	use intcode::IntCodeError;

	use super::*;
	use std::fs;

	#[test]
	fn part_1_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input)?.first_try_painted_tiles, 2255);

		Ok(())
	}

	#[test]
	fn part_2_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		let expected = r#"
⬛⬜️⬜️⬜️⬛⬛⬛⬜️⬜️⬛⬛⬜️⬛⬛⬜️⬛⬜️⬜️⬜️⬜️⬛⬜️⬜️⬜️⬛⬛⬛⬜️⬜️⬛⬛⬜️⬜️⬜️⬛⬛⬛⬜️⬜️⬛⬛⬛⬛
⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬜️⬛⬜️⬛⬜️⬛⬛⬜️⬛⬛⬛⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬜️⬛⬛⬛
⬛⬜️⬜️⬜️⬛⬛⬜️⬛⬛⬛⬛⬜️⬜️⬛⬛⬛⬜️⬜️⬜️⬛⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬛⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬜️⬛⬛⬛
⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬛⬛⬜️⬛⬜️⬛⬛⬜️⬛⬛⬛⬛⬜️⬜️⬜️⬛⬛⬜️⬛⬛⬛⬛⬜️⬜️⬜️⬛⬛⬜️⬜️⬜️⬜️⬛⬛⬛
⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬜️⬛⬜️⬛⬜️⬛⬛⬜️⬛⬛⬛⬛⬜️⬛⬛⬛⬛⬜️⬛⬛⬜️⬛⬜️⬛⬜️⬛⬛⬜️⬛⬛⬜️⬛⬛⬛
⬛⬜️⬜️⬜️⬛⬛⬛⬜️⬜️⬛⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬛⬛⬜️⬛⬛⬛⬛⬛⬜️⬜️⬛⬛⬜️⬛⬛⬜️⬛⬜️⬛⬛⬜️⬛⬛⬛
"#
		.trim_start()
		.to_string();

		assert_eq!(solve(&input)?.registration_identifier, expected);

		Ok(())
	}
}
