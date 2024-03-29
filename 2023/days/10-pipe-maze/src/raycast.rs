use crate::{follow_facing_pipe, sym, Solution};
use aoc_utils::direction::*;

#[derive(Debug)]
struct Maze<'a> {
	map: &'a [u8],
	line_width: usize,
	height: usize,
	start: usize,
}

impl<'a> Maze<'a> {
	fn new(map: &'a [u8]) -> Self {
		let width = map
			.iter()
			.position(|b| *b == sym::NEWLINE)
			.expect("maze map must have at least one newline");
		let line_width = width + 1;
		let height = map.len() / line_width;
		let start = map
			.iter()
			.position(|b| *b == sym::START)
			.expect("maze map must have start position");

		Self {
			map,
			height,
			line_width,
			start,
		}
	}

	fn start_running(&self) -> MazeRunner {
		let mut runner = [
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
				on_vertical: false,
				facing: dir,
				x: (self.start % self.line_width) as isize,
				y: self.start / self.line_width,
			})
		})
		.expect("expected 1 (of 2) start connections");

		// if one component is south it must be the first element we find due to search array order
		runner.on_vertical = matches!(runner.facing, South);

		runner
	}
}

#[derive(Debug, Clone)]
struct MazeRunner {
	position: usize,
	distance: usize,
	facing: Direction,
	on_vertical: bool,
	y: usize,
	x: isize,
}

#[derive(Debug, Clone)]
struct LoopPart {
	x_position: u16,
	vert_wall: bool,
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

		if next_position == maze.start {
			return true;
		}

		let next_b = maze.map[next_position];

		let next_facing = follow_facing_pipe(&self.facing, next_b)
			.expect("expected to be able to follow all pipes");

		self.position = next_position;
		// NOTE: we count any element that has a bottom/southern vertical part as an
		// edge, so | or F or 7.
		// This distinction of deciding for top or bottom counting as edge was super
		// important to get rid of the problem to have the raycasting happen _on_
		// the polygon vertex, making the problem unsolvable in this manner.
		self.on_vertical = matches!(next_b, sym::VERT | sym::BOT_LEFT | sym::BOT_RIGHT);
		self.facing = next_facing;
		self.y = next_y;
		self.x = next_x;
		false
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let maze = Maze::new(input);

	// record of all loop parts and vertical intersections of the maze
	let mut scan_lines: Vec<Vec<LoopPart>> = vec![vec![]; maze.height];

	let mut runner = maze.start_running();

	let furthest_distance = loop {
		scan_lines[runner.y].push(LoopPart {
			x_position: runner.x as u16,
			vert_wall: runner.on_vertical,
		});

		if runner.follow_pipe_or_end(&maze) {
			break (runner.distance / 2);
		}
	};

	let mut points_inside = 0;

	for mut scan_line in scan_lines {
		let mut odd_edge = false;
		scan_line.sort_unstable_by_key(|s| s.x_position);
		for window in scan_line.windows(2) {
			if window[0].vert_wall {
				odd_edge = !odd_edge
			}
			if odd_edge {
				points_inside += (window[1].x_position - window[0].x_position - 1) as usize;
			}
		}
	}

	Solution(furthest_distance, points_inside)
}
