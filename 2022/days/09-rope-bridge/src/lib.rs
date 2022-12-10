use std::{
	collections::HashSet,
	ops::{Add, AddAssign, Sub},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(u64, u64);

/// Point(x,y)
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point(i16, i16);

impl Point {
	fn abs_diff(self, other: Self) -> u16 {
		let Point(x, y) = self;
		let Point(x2, y2) = other;
		(x.abs_diff(x2)).max(y.abs_diff(y2))
	}

	fn to_dir(self) -> Point {
		let Point(x, y) = self;
		Point(x.clamp(-1, 1), y.clamp(-1, 1))
	}
}

impl From<u8> for Point {
	fn from(value: u8) -> Self {
		match value {
			b'U' => Point(0, 1),
			b'R' => Point(1, 0),
			b'L' => Point(-1, 0),
			b'D' => Point(0, -1),
			_ => panic!("bad input, expected direction (LRUD)"),
		}
	}
}

impl Add for Point {
	type Output = Point;

	fn add(self, rhs: Self) -> Self::Output {
		let Point(x, y) = self;
		let Point(x2, y2) = rhs;
		Point(x + x2, y + y2)
	}
}

impl AddAssign for Point {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sub for Point {
	type Output = Point;

	fn sub(self, rhs: Self) -> Self::Output {
		let Point(x1, y1) = self;
		let Point(x2, y2) = rhs;
		Point(x1 - x2, y1 - y2)
	}
}

const NUMBERS_START: u8 = b'0';
fn parse_usize(b: &[u8]) -> usize {
	let mut num = (b[0] - NUMBERS_START) as usize;
	for digit in &b[1..] {
		if !digit.is_ascii_digit() {
			return num;
		}
		num = num * 10 + (digit - NUMBERS_START) as usize;
	}
	num
}

const START_POINT: Point = Point(0, 0);

pub fn solve(input: &[u8]) -> Solution {
	let mut head = START_POINT;
	let mut rope = [START_POINT; 9];
	let mut tail_visited: HashSet<Point> = HashSet::from([START_POINT]);
	let mut long_tail_visited: HashSet<Point> = HashSet::from([START_POINT]);

	for line in input.split(|b| *b == b'\n') {
		let dir: Point = line[0].into();

		let steps = parse_usize(&line[2..]);

		for _ in 0..steps {
			head += dir;

			let mut current_head = head;

			for (index, rope_part) in rope.iter_mut().enumerate() {
				if rope_part.abs_diff(current_head) <= 1 {
					break;
				}

				*rope_part += (current_head - *rope_part).to_dir();
				current_head = *rope_part;

				match index {
					0 => {
						tail_visited.insert(*rope_part);
					}
					8 => {
						long_tail_visited.insert(*rope_part);
					}
					_ => (),
				}
			}
		}
	}

	Solution(tail_visited.len() as u64, long_tail_visited.len() as u64)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(13,1))]
	#[case(include_bytes!("../inputs/example_large.txt"), Solution(88,36))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(6367,2536))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
