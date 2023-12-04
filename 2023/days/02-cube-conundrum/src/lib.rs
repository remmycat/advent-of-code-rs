use aoc_2023_utils::ascii_int::parse_u8_unchecked;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

const LINE_SEP: u8 = b'\n';
const GAME_SEP: u8 = b':';
const GRAB_SEP: u8 = b';';
const COLOR_SEP: u8 = b',';
const NUM_SEP: u8 = b' ';

enum CubeColor {
	Red,
	Green,
	Blue,
}

impl From<&[u8]> for CubeColor {
	fn from(color_str: &[u8]) -> Self {
		match color_str.len() {
			3 => Self::Red,
			5 => Self::Green,
			4 => Self::Blue,
			_ => panic!("bad color input"),
		}
	}
}

struct CubesOfColor {
	color: CubeColor,
	count: u8,
}

impl From<&[u8]> for CubesOfColor {
	fn from(span: &[u8]) -> Self {
		// e.g. " 4 red" or " 15 blue"
		let mut parts = span[1..].split(|b| *b == NUM_SEP);
		let (digits, color_str) = (
			parts.next().expect("bad input, expected cube count"),
			parts.next().expect("bad input, expected cube color"),
		);

		Self {
			color: CubeColor::from(color_str),
			count: parse_u8_unchecked(digits),
		}
	}
}

#[derive(Debug, Default)]
struct CubeSet {
	reds: u8,
	greens: u8,
	blues: u8,
}

const SAMPLE_SET: CubeSet = CubeSet {
	reds: 12,
	greens: 13,
	blues: 14,
};

impl CubeSet {
	fn possible_from_sample(&self) -> bool {
		self.reds <= SAMPLE_SET.reds
			&& self.greens <= SAMPLE_SET.greens
			&& self.blues <= SAMPLE_SET.blues
	}
	fn power(&self) -> usize {
		self.reds as usize * self.greens as usize * self.blues as usize
	}
}

impl From<&[u8]> for CubeSet {
	fn from(span: &[u8]) -> Self {
		// e.g. " 3 blue, 4 red"
		let mut set = CubeSet::default();
		for cubes in span.split(|b| *b == COLOR_SEP).map(CubesOfColor::from) {
			match cubes.color {
				CubeColor::Red => {
					set.reds += cubes.count;
				}
				CubeColor::Green => {
					set.greens += cubes.count;
				}
				CubeColor::Blue => {
					set.blues += cubes.count;
				}
			}
		}

		set
	}
}

struct Game {
	min_set: CubeSet,
}

impl From<&[u8]> for Game {
	fn from(span: &[u8]) -> Self {
		let grabs = span
			.split(|b| *b == GAME_SEP)
			.nth(1)
			.expect("bad input, expected game grabs");

		let min_set: CubeSet = grabs.split(|b| *b == GRAB_SEP).map(CubeSet::from).fold(
			CubeSet::default(),
			|mut min_set: CubeSet, grab| {
				min_set.reds = min_set.reds.max(grab.reds);
				min_set.greens = min_set.greens.max(grab.greens);
				min_set.blues = min_set.blues.max(grab.blues);

				min_set
			},
		);

		Self { min_set }
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let games: Vec<_> = input
		.split(|b| *b == LINE_SEP)
		.filter(|line| !line.is_empty())
		.map(Game::from)
		.collect();

	let possible_check_id_sum: usize = games
		.iter()
		.enumerate()
		.filter(|(_, game)| game.min_set.possible_from_sample())
		.map(|(index, _)| index + 1)
		.sum();

	let min_set_powers: usize = games.iter().map(|game| game.min_set.power()).sum();

	Solution(possible_check_id_sum, min_set_powers)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(8,2286))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(2285,77021))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
