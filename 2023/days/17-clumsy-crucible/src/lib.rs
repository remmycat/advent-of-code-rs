use std::iter::successors;

use aoc_utils::direction::*;
use pathfinding::prelude::astar;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i16, i16);

impl Point {
	fn distance(&self, other: &Point) -> u16 {
		self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
	}

	fn neighbour(&self, direction: &Direction) -> Point {
		match direction {
			North => Point(self.0, self.1 - 1),
			East => Point(self.0 + 1, self.1),
			South => Point(self.0, self.1 + 1),
			West => Point(self.0 - 1, self.1),
		}
	}
}

fn find_lowest_heat_loss(grid: &Grid, min_step: usize, max_step: usize) -> u16 {
	let start_point = Point(0, 0);
	let end_point = Point(grid.width - 1, grid.height - 1);
	let start_direction: Option<Direction> = None;
	let successor_count = max_step - min_step + 1;

	let (_, lowest_loss) = astar(
		&(start_point, start_direction),
		|(point, facing)| {
			// "facing == None" means we're just starting and can go south or east!
			let (widdershins, clockwise) = facing
				.map(|dir| (dir.turn_widdershins(), dir.turn_clockwise()))
				.unwrap_or((South, East));

			let first_widdershins = grid.get_point_and_heat(point.neighbour(&widdershins));

			let widdershins_points = successors(first_widdershins, move |(point, cost)| {
				grid.get_point_and_heat(point.neighbour(&widdershins))
					.map(|(next_point, next_cost)| (next_point, cost + next_cost))
			})
			.skip(min_step - 1)
			.take(successor_count)
			.map(move |(point, cost)| ((point, Some(widdershins)), cost));

			let first_clockwise = grid.get_point_and_heat(point.neighbour(&clockwise));

			let clock_points = successors(first_clockwise, move |(point, cost)| {
				grid.get_point_and_heat(point.neighbour(&clockwise))
					.map(|(next_point, next_cost)| (next_point, cost + next_cost))
			})
			.skip(min_step - 1)
			.take(successor_count)
			.map(move |(point, cost)| ((point, Some(clockwise)), cost));

			widdershins_points.chain(clock_points)
		},
		|(point, _)| point.distance(&end_point),
		|(point, _)| *point == end_point,
	)
	.expect("must have shortest path");

	lowest_loss
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

	fn is_valid_point(&self, &Point(x, y): &Point) -> bool {
		x >= 0 && x < self.width && y >= 0 && y < self.height
	}

	fn get_heat_loss_at(&self, &Point(x, y): &Point) -> u16 {
		let index = (y * self.line_width + x) as usize;
		(self.input[index] - b'0') as u16
	}

	fn get_point_and_heat(&self, point: Point) -> Option<(Point, u16)> {
		if self.is_valid_point(&point) {
			let heat_loss = self.get_heat_loss_at(&point);
			Some((point, heat_loss))
		} else {
			None
		}
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let grid = Grid::parse(input);

	let lowest_heat_loss = find_lowest_heat_loss(&grid, 1, 3);
	let lowest_heat_loss_ultra_crucible = find_lowest_heat_loss(&grid, 4, 10);

	Solution(
		lowest_heat_loss as usize,
		lowest_heat_loss_ultra_crucible as usize,
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(102,94),
	)]
	#[case::example_2(
		include_bytes!("../inputs/example_2.txt"),
		Solution(59,71),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(686,801),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
