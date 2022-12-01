use std::str::FromStr;

use intcode::{IntCodeError, IntCodeProgram};
use screen::{Screen, Tile};

mod screen;

pub struct Solution {
	pub block_tiles: usize,
}

pub fn solve(input: &str) -> Result<Solution, IntCodeError> {
	let software = IntCodeProgram::from_str(input)?;
	let mut screen = Screen::new();

	screen.run(software)?;

	let block_tiles = screen
		.grid
		.values()
		.filter(|tile| matches!(tile, Tile::Block))
		.count();

	Ok(Solution { block_tiles })
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_solution() -> Result<(), IntCodeError> {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input)?.block_tiles, 0);

		Ok(())
	}

	#[test]
	fn part_2_example_cases() {
		unimplemented!();
	}

	#[test]
	fn part_2_solution() {
		unimplemented!();
	}
}
