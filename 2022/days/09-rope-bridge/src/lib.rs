use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(u64, u64);

mod assumptions {
	pub type GridInt = i16;
	pub type GridUint = u16;
}

/// Point(x,y)
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point(assumptions::GridInt, assumptions::GridInt);

impl Point {
	fn abs_diff(self, other: Self) -> assumptions::GridUint {
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

impl Ord for Point {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.cmp(&other.0).then(self.1.cmp(&other.1))
	}
}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

const GRID_SIZE: usize = 128;
const GRID_DIV: assumptions::GridInt = GRID_SIZE as assumptions::GridInt;

/// Stores a 2d grid of bool values as binary inside `u128`s.
/// These are grouped in "chunks", each covering a 128x128 square of the grid.
#[derive(Debug)]
struct CondensedBooleanGrid {
	chunks: Vec<(Point, [u128; GRID_SIZE])>,
}

impl CondensedBooleanGrid {
	fn new() -> Self {
		Self { chunks: vec![] }
	}

	fn get_chunk_index(p: Point) -> Point {
		// the -1 for negative is to have only one chunk be responsible for x=0 / y=0
		let x_div = p.0 / GRID_DIV - p.0.is_negative() as assumptions::GridInt;
		let y_div = p.1 / GRID_DIV - p.1.is_negative() as assumptions::GridInt;
		Point(x_div, y_div)
	}

	fn get_local_coords(p: Point) -> (u128, usize) {
		let x_local = (p.0 % GRID_DIV).unsigned_abs() as u128;
		let y_local = (p.1 % GRID_DIV).unsigned_abs() as usize;

		(x_local, y_local)
	}

	fn get_chunk(&mut self, index: &Point) -> &mut [u128; GRID_SIZE] {
		match self.chunks.binary_search_by_key(&index, |(p, _)| p) {
			Ok(pos) => &mut self.chunks.get_mut(pos).expect("impossible").1,
			Err(pos) => {
				self.chunks.insert(pos, (index.to_owned(), [0; GRID_SIZE]));
				&mut self.chunks.get_mut(pos).expect("impossible").1
			}
		}
	}

	fn set_true(&mut self, point: Point) {
		let chunk_index = CondensedBooleanGrid::get_chunk_index(point);
		let chunk = self.get_chunk(&chunk_index);

		let (x, y) = CondensedBooleanGrid::get_local_coords(point);
		chunk[y] |= 1_u128 << x;
	}

	fn count(&self) -> usize {
		self.chunks
			.iter()
			.flat_map(|(_, chunk)| chunk.iter().map(|e| e.count_ones() as usize))
			.sum()
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
	let mut tail_visited: CondensedBooleanGrid = CondensedBooleanGrid::new();
	let mut long_tail_visited: CondensedBooleanGrid = CondensedBooleanGrid::new();

	tail_visited.set_true(START_POINT);
	long_tail_visited.set_true(START_POINT);

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
						tail_visited.set_true(*rope_part);
					}
					8 => {
						long_tail_visited.set_true(*rope_part);
					}
					_ => (),
				}
			}
		}
	}

	Solution(
		tail_visited.count() as u64,
		long_tail_visited.count() as u64,
	)
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
