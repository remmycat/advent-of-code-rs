use aoc_utils::direction::*;
// use pathfinding::prelude::yen;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i16, i16);

impl Point {
	// fn distance(&self, other: &Point) -> u16 {
	// 	self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
	// }

	fn neighbour(&self, direction: &Direction) -> Point {
		match direction {
			North => Point(self.0, self.1 - 1),
			East => Point(self.0 + 1, self.1),
			South => Point(self.0, self.1 + 1),
			West => Point(self.0 - 1, self.1),
		}
	}
}

const DIRECTIONS: [Direction; 4] = [North, East, South, West];

fn find_paths(
	grid: &Grid,
	goal: &Point,
	max_path_length: &mut usize,
	current_point: Point,
	current_dir: Direction,
	current_len: usize,
) {
	let backwards = current_dir.opposite();

	let mut succ: Vec<_> = DIRECTIONS
		.into_iter()
		.filter_map(|next_dir| {
			if next_dir == backwards {
				return None;
			}

			let next = current_point.neighbour(&next_dir);
			if grid.validate_point(&next, &next_dir) {
				Some(((next, next_dir), current_len + 1))
			} else {
				None
			}
		})
		.collect();

	// Follow path until next fork
	while succ.len() == 1 {
		let ((succ_point, succ_dir), succ_len) = succ.pop().unwrap();
		if succ_point == *goal {
			*max_path_length = (*max_path_length).max(succ_len);
			return;
		}

		let backwards = succ_dir.opposite();

		// println!("PT: {succ_point:?}, dir: {succ_dir:?}, len: {succ_len}");
		succ.extend(DIRECTIONS.into_iter().filter_map(|next_dir| {
			if next_dir == backwards {
				return None;
			}

			let next = succ_point.neighbour(&next_dir);
			if grid.validate_point(&next, &next_dir) {
				Some(((next, next_dir), succ_len + 1))
			} else {
				None
			}
		}));

		if succ.is_empty() {
			panic!("did not expect cul-de-sacs");
		}
	}

	for ((point, dir), len) in succ {
		find_paths(grid, goal, max_path_length, point, dir, len)
	}
}

fn find_paths_unsloped(
	grid: &Grid,
	goal: &Point,
	max_path_length: &mut usize,
	mut fork_points: Vec<Point>,
	current_point: Point,
	current_dir: Direction,
	current_len: usize,
) {
	let backwards = current_dir.opposite();

	let mut succ: Vec<_> = DIRECTIONS
		.into_iter()
		.filter_map(|next_dir| {
			if next_dir == backwards {
				return None;
			}

			let next = current_point.neighbour(&next_dir);

			if fork_points.contains(&next) || !grid.validate_point_unsloped(&next) {
				None
			} else {
				Some(((next, next_dir), current_len + 1))
			}
		})
		.collect();

	// Follow path until next fork
	while succ.len() == 1 {
		let ((succ_point, succ_dir), succ_len) = succ.pop().unwrap();
		if succ_point == *goal {
			*max_path_length = (*max_path_length).max(succ_len);
			return;
		}

		let backwards = succ_dir.opposite();

		// println!("PT: {succ_point:?}, dir: {succ_dir:?}, len: {succ_len}");
		succ.extend(DIRECTIONS.into_iter().filter_map(|next_dir| {
			if next_dir == backwards {
				return None;
			}

			let next = succ_point.neighbour(&next_dir);
			if !grid.validate_point_unsloped(&next) {
				None
			} else {
				Some(((next, next_dir), succ_len + 1))
			}
		}));

		if succ.is_empty() {
			// dead end
			return;
		}

		if succ.len() > 1 {
			if fork_points.contains(&succ_point) {
				// dead end (already crossed here!)
				return;
			}
			fork_points.push(succ_point);
		}
	}

	for ((point, dir), len) in succ {
		find_paths_unsloped(
			grid,
			goal,
			max_path_length,
			fork_points.clone(),
			point,
			dir,
			len,
		)
	}
}

fn find_longest_path(grid: &Grid, sloped: bool) -> usize {
	let start_point = Point(1, 0);
	let end_point = Point(grid.width - 2, grid.height - 1);
	let start_direction: Direction = South;

	let mut max_path_length: usize = 0;

	if sloped {
		find_paths(
			grid,
			&end_point,
			&mut max_path_length,
			start_point,
			start_direction,
			0,
		);
	} else {
		find_paths_unsloped(
			grid,
			&end_point,
			&mut max_path_length,
			vec![],
			start_point,
			start_direction,
			0,
		);
	}

	// let paths = yen(
	// 	&(start_point, start_direction),
	// 	|(point, dir)| {
	// 		let backwards = dir.opposite();
	// 		let mut succ: Vec<_> = [
	// 			((point.neighbour(&North), North), 1),
	// 			((point.neighbour(&East), East), 1),
	// 			((point.neighbour(&South), South), 1),
	// 			((point.neighbour(&West), West), 1),
	// 		]
	// 		.into_iter()
	// 		.filter(|((point, in_direction), _)| {
	// 			*in_direction != backwards && grid.validate_point(point, in_direction)
	// 		})
	// 		.collect();

	// 		while succ.len() == 1 {
	// 			let ((point, dir), cost) = succ.pop().unwrap();
	// 			let backwards = dir.opposite();
	// 			succ.extend(
	// 				[
	// 					((point.neighbour(&North), North), cost + 1),
	// 					((point.neighbour(&East), East), cost + 1),
	// 					((point.neighbour(&South), South), cost + 1),
	// 					((point.neighbour(&West), West), cost + 1),
	// 				]
	// 				.into_iter()
	// 				.filter(|((point, in_direction), _)| {
	// 					*in_direction != backwards && grid.validate_point(point, in_direction)
	// 				}),
	// 			);

	// 			if succ.is_empty() {
	// 				succ = vec![((point, dir), cost)];
	// 				break;
	// 			}
	// 		}
	// 		succ
	// 	},
	// 	|(point, _)| *point == end_point,
	// 	100000,
	// );

	max_path_length
}

struct Grid<'i> {
	input: &'i [u8],
	line_width: i16,
	width: i16,
	height: i16,
}

impl<'i> Grid<'i> {
	fn parse(input: &'i [u8]) -> Self {
		let width = input
			.iter()
			.position(|b| *b == b'\n')
			.expect("input has at least one newline");
		let line_width = width + 1;
		let height = input.len() / (line_width);
		Self {
			input,
			height: (height).try_into().expect("height fits into i16"),
			width: (width).try_into().expect("width fits into i16"),
			line_width: (line_width).try_into().expect("line_width fits into i16"),
		}
	}

	fn is_valid_coordinate(&self, &Point(x, y): &Point) -> bool {
		x >= 0 && x < self.width && y >= 0 && y < self.height
	}

	fn get_tile_at(&self, &Point(x, y): &Point) -> &u8 {
		// println!("tile at {x},{y}");
		let index = (y * self.line_width + x) as usize;
		&self.input[index]
	}

	fn validate_point(&self, point: &Point, in_direction: &Direction) -> bool {
		if self.is_valid_coordinate(point) {
			let tile = self.get_tile_at(point);

			// println!("Checking {tile}, {in_direction:?}");

			matches!(
				(tile, in_direction),
				(b'.', _) | (b'>', East) | (b'v', South) | (b'<', West) | (b'^', North)
			)
		} else {
			false
		}
	}

	fn validate_point_unsloped(&self, point: &Point) -> bool {
		if self.is_valid_coordinate(point) {
			let tile = self.get_tile_at(point);

			*tile != b'#'
		} else {
			false
		}
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let grid = Grid::parse(input);

	let longest_path = find_longest_path(&grid, true);

	let longest_path_unsloped = find_longest_path(&grid, false);

	Solution(longest_path, longest_path_unsloped)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(94,154),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(2294,6418),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
