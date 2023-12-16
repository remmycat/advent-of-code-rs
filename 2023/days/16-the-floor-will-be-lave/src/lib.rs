#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug)]
struct Map {
	tiles: Vec<Tile>,
	width: usize,
	height: usize,
	e_max: usize,
	s_max: usize,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
	VertSplitter,
	HorzSplitter,
	DiagUphill,
	DiagDownhill,
	Nothing,
}

impl Tile {
	fn from_u8(b: &u8) -> Self {
		match *b {
			b'|' => Self::VertSplitter,
			b'-' => Self::HorzSplitter,
			b'/' => Self::DiagUphill,
			b'\\' => Self::DiagDownhill,
			_ => Self::Nothing,
		}
	}
}

impl Map {
	fn new(input: &[u8]) -> Self {
		let width = input
			.iter()
			.position(|b| *b == b'\n')
			.expect("must have at least 1 newline");
		let line_width = width + 1;
		let height = input.len() / line_width;

		let mut tiles = Vec::with_capacity(width * height);
		for y in 0..height {
			let start = y * line_width;
			let end = start + width;
			tiles.extend(input[start..end].iter().map(Tile::from_u8));
		}

		let len = tiles.len();

		Self {
			tiles,
			width,
			height,
			e_max: len - 1,
			s_max: len - width - 1,
		}
	}

	fn next_in_front_of_me(&self, pos: usize, facing: Direction) -> Option<usize> {
		match facing {
			North if pos > self.width => Some(pos - self.width),
			East if (pos + 1) % self.width != 0 && pos < self.e_max => Some(pos + 1),
			South if pos < self.s_max => Some(pos + self.width),
			West if pos % self.width != 0 => Some(pos - 1),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	North,
	East,
	South,
	West,
}

use Direction::*;

#[derive(Debug, Clone, Copy, Default)]
struct Breadcrumb(u8);

const BIT_NORTH: u8 = 0b0001;
const BIT_EAST: u8 = 0b0010;
const BIT_SOUTH: u8 = 0b0100;
const BIT_WEST: u8 = 0b1000;

impl Breadcrumb {
	fn dir_bit(dir: Direction) -> u8 {
		match dir {
			North => BIT_NORTH,
			East => BIT_EAST,
			South => BIT_SOUTH,
			West => BIT_WEST,
		}
	}

	fn insert(&mut self, dir: Direction) -> bool {
		let dir_bit = Breadcrumb::dir_bit(dir);
		if self.0 & dir_bit != 0 {
			false
		} else {
			self.0 |= dir_bit;
			true
		}
	}

	fn is_empty(&self) -> bool {
		self.0 == 0
	}
}

#[derive(Debug, Clone)]
struct MazeRunner<'a> {
	map: &'a Map,
	position: usize,
	facing: Direction,
}

impl<'a> MazeRunner<'a> {
	fn run_to(
		&mut self,
		next_position: usize,
		breadcrumbs: &mut [Breadcrumb],
	) -> Option<Option<Self>> {
		let put_down = breadcrumbs[next_position].insert(self.facing);
		if !put_down {
			// we can stop, since we went to this spot in this direction already
			// println!("saved!");
			return None;
		}

		self.position = next_position;

		match (self.facing, self.map.tiles[next_position]) {
			(North | South, Tile::HorzSplitter) => {
				let mut other = self.clone();
				self.facing = East;
				other.facing = West;
				Some(Some(other))
			}
			(East | West, Tile::VertSplitter) => {
				let mut other = self.clone();
				self.facing = North;
				other.facing = South;
				Some(Some(other))
			}
			(East, Tile::DiagUphill) | (West, Tile::DiagDownhill) => {
				self.facing = North;
				Some(None)
			}
			(East, Tile::DiagDownhill) | (West, Tile::DiagUphill) => {
				self.facing = South;
				Some(None)
			}
			(North, Tile::DiagUphill) | (South, Tile::DiagDownhill) => {
				self.facing = East;
				Some(None)
			}
			(North, Tile::DiagDownhill) | (South, Tile::DiagUphill) => {
				self.facing = West;
				Some(None)
			}
			_ => Some(None),
		}
	}

	fn run(&mut self, breadcrumbs: &mut [Breadcrumb]) -> Option<Option<Self>> {
		let next_position = self.map.next_in_front_of_me(self.position, self.facing)?;
		self.run_to(next_position, breadcrumbs)
	}
}

const EMPTY_BREADCRUMB: Breadcrumb = Breadcrumb(0);

fn follow_to_end(mut runner: MazeRunner, breadcrumbs: &mut [Breadcrumb]) {
	match runner.run(breadcrumbs) {
		None => (),
		Some(None) => follow_to_end(runner, breadcrumbs),
		Some(Some(other)) => {
			follow_to_end(other, breadcrumbs);
			follow_to_end(runner, breadcrumbs);
		}
	}
}

fn run_the_maze(map: &Map, breadcrumbs: &mut [Breadcrumb], start_pos: usize, start_dir: Direction) {
	let mut runner = MazeRunner {
		map,
		position: start_pos,
		facing: start_dir,
	};

	// runner should start at -1, but signed ints are annoying
	// so we "fake it"" by starting at 0, and telling it to run to 0.

	match runner.run_to(start_pos, breadcrumbs) {
		None => (),
		Some(None) => follow_to_end(runner, breadcrumbs),
		Some(Some(other)) => {
			follow_to_end(other, breadcrumbs);
			follow_to_end(runner, breadcrumbs);
		}
	}
}

fn count_reset_breadcrumbs(crumbs: &mut [Breadcrumb]) -> usize {
	let mut sum: usize = 0;

	for crumb in crumbs.iter_mut() {
		if !crumb.is_empty() {
			sum += 1;
			crumb.0 = 0;
		}
	}

	sum
}

pub fn solve(input: &[u8]) -> Solution {
	let map = Map::new(input);
	let mut breadcrumbs: Vec<Breadcrumb> = vec![EMPTY_BREADCRUMB; map.width * map.height];

	// first run at top left
	run_the_maze(&map, &mut breadcrumbs, 0, East);
	let energized_top_left = count_reset_breadcrumbs(&mut breadcrumbs);

	// todo skip first and set to energized_top_left
	let mut energized_max = 0;

	for i in 0..map.height {
		run_the_maze(&map, &mut breadcrumbs, i * map.width, East);
		let energized = count_reset_breadcrumbs(&mut breadcrumbs);
		energized_max = energized_max.max(energized);

		run_the_maze(&map, &mut breadcrumbs, (i + 1) * map.width - 1, West);
		let energized = count_reset_breadcrumbs(&mut breadcrumbs);
		energized_max = energized_max.max(energized);
	}

	for i in 0..map.width {
		run_the_maze(&map, &mut breadcrumbs, i, South);
		let energized = count_reset_breadcrumbs(&mut breadcrumbs);
		energized_max = energized_max.max(energized);

		run_the_maze(
			&map,
			&mut breadcrumbs,
			(map.height - 1) * map.width + i,
			North,
		);
		let energized = count_reset_breadcrumbs(&mut breadcrumbs);
		energized_max = energized_max.max(energized);
	}

	// for (i, crumb) in breadcrumbs.iter().enumerate() {
	// 	if (i + 1) % map.width == 0 {
	// 		if crumb.0 > 0 {
	// 			println!("#");
	// 		} else {
	// 			println!(".")
	// 		}
	// 	} else {
	// 		if crumb.0 > 0 {
	// 			print!("#");
	// 		} else {
	// 			print!(".")
	// 		}

	// 		// print!("{:>2} ", crumb.0);
	// 	}
	// }
	// //
	// let energized_top_left = count_reset_breadcrumbs(&mut breadcrumbs);

	Solution(energized_top_left, energized_max)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(include_bytes!("../inputs/example.txt"), Solution(46,51))]
	#[case::personal(include_bytes!("../inputs/personal.txt"), Solution(8249,8444))] // 8244/8245 too low
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
