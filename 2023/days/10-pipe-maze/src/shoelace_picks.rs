use crate::{follow_facing_pipe, sym, Solution};
use aoc_utils::direction::*;

#[derive(Debug)]
struct Maze<'a> {
	map: &'a [u8],
	line_width: usize,
	start: usize,
}

impl<'a> Maze<'a> {
	fn new(map: &'a [u8]) -> Self {
		let width = map
			.iter()
			.position(|b| *b == sym::NEWLINE)
			.expect("maze map must have at least one newline");
		let line_width = width + 1;
		let start = map
			.iter()
			.position(|b| *b == sym::START)
			.expect("maze map must have start position");

		Self {
			map,
			line_width,
			start,
		}
	}

	fn start_running(&self) -> MazeRunner {
		[
			(South, Some(self.start + self.line_width)),
			(North, self.start.checked_sub(self.line_width)),
			(East, Some(self.start + 1)),
			// (West, self.start.checked_sub(1)),
		]
		.into_iter()
		.find_map(|(dir, index)| {
			let index = index?;
			if index >= self.map.len() {
				return None;
			}

			// assure this facing direction is leading to a next location
			follow_facing_pipe(&dir, self.map[index])?;

			Some(MazeRunner {
				position: self.start,
				distance: 0,
				shoelace_sum: 0,
				facing: dir,
				x: (self.start % self.line_width) as isize,
				y: (self.start / self.line_width) as isize,
			})
		})
		.expect("expected 1 (of 2) start connections")
	}
}

#[derive(Debug, Clone)]
struct MazeRunner {
	position: usize,
	distance: usize,
	facing: Direction,
	shoelace_sum: isize,
	y: isize,
	x: isize,
}

impl MazeRunner {
	fn follow_pipe_or_end(&mut self, maze: &Maze) -> bool {
		let (next_position, next_x, next_y) = match self.facing {
			North => (self.position - maze.line_width, self.x, self.y - 1),
			East => (self.position + 1, self.x + 1, self.y),
			South => (self.position + maze.line_width, self.x, self.y + 1),
			West => (self.position - 1, self.x - 1, self.y),
		};

		self.distance += 1;
		self.shoelace_sum += (self.x - next_x) * (self.y + next_y);

		if next_position == maze.start {
			return true;
		}

		let next_b = maze.map[next_position];

		let next_facing = follow_facing_pipe(&self.facing, next_b)
			.expect("expected to be able to follow all pipes");

		self.position = next_position;
		self.facing = next_facing;
		self.y = next_y;
		self.x = next_x;
		false
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let maze = Maze::new(input);

	let mut runner = maze.start_running();

	// Shoelace / Determinants  (x1 - x2) * (y1 + y2)  +   (x2 - x3) * (y2 + y3)  +   ...  +  (xn - x1) * (yn + y1)

	let (loop_length, shoelace_sum) = loop {
		if runner.follow_pipe_or_end(&maze) {
			break (runner.distance, runner.shoelace_sum);
		}
	};

	let area = shoelace_sum.unsigned_abs() / 2;

	// always even
	let furthest_distance = loop_length / 2;

	// picks theorem
	let points_inside = area - furthest_distance + 1;

	Solution(furthest_distance, points_inside)
}
