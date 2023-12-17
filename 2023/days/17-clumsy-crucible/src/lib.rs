use aoc_utils::direction::*;
use pathfinding::prelude::astar;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
	row: i16,
	col: i16,
}

impl Pos {
	fn distance(&self, other: &Pos) -> u16 {
		self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
	}

	fn if_coming_from(&self, coming_from: Direction) -> Pos {
		match coming_from {
			North => Pos {
				row: self.row + 1,
				col: self.col,
			},
			South => Pos {
				row: self.row - 1,
				col: self.col,
			},
			East => Pos {
				row: self.row,
				col: self.col - 1,
			},
			West => Pos {
				row: self.row,
				col: self.col + 1,
			},
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
	coming_from: Direction,
	straight_count: u8,
}

impl State {
	fn if_coming_from(&self, coming_from: Direction) -> State {
		State {
			coming_from,
			straight_count: if coming_from == self.coming_from {
				self.straight_count + 1
			} else {
				1
			},
		}
	}
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

	fn is_valid_pos(&self, pos: &Pos) -> bool {
		pos.row >= 0 && pos.row < self.height && pos.col >= 0 && pos.col < self.width
	}

	fn get_cost_at(&self, pos: &Pos) -> u16 {
		let index = (pos.row * self.line_width + pos.col) as usize;
		(self.input[index] - b'0') as u16
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let grid = Grid::parse(input);
	let gridref = &grid;

	let start = (
		Pos { row: 0, col: 0 },
		State {
			coming_from: West,
			straight_count: 0,
		},
	);

	let goal = Pos {
		row: grid.height - 1,
		col: grid.width - 1,
	};

	let (_, lowest_cost) = astar(
		&start,
		|(pos, state)| {
			let has_to_turn = state.straight_count == 3;
			let moving_forwards = state.coming_from;
			let moving_backwards = state.coming_from.opposite();
			[
				(pos.if_coming_from(North), state.if_coming_from(North)),
				(pos.if_coming_from(East), state.if_coming_from(East)),
				(pos.if_coming_from(South), state.if_coming_from(South)),
				(pos.if_coming_from(West), state.if_coming_from(West)),
			]
			.into_iter()
			.filter(move |(next_pos, next_state)| {
				next_state.coming_from != (moving_backwards)
					&& (!has_to_turn || next_state.coming_from != moving_forwards)
					&& gridref.is_valid_pos(next_pos)
			})
			.map(|(pos, state)| {
				let cost = gridref.get_cost_at(&pos);
				((pos, state), cost)
			})
		},
		|(pos, _)| pos.distance(&goal),
		|(pos, _)| *pos == goal,
	)
	.expect("must have shortest path");

	let (_, lowest_cost_ultra_crucible) = astar(
		&start,
		|(pos, state)| {
			let can_turn = state.straight_count >= 4;
			let has_to_turn = state.straight_count == 10;
			let moving_forwards = state.coming_from;
			let moving_backwards = state.coming_from.opposite();
			let is_start = state.straight_count == 0;
			[
				(pos.if_coming_from(North), state.if_coming_from(North)),
				(pos.if_coming_from(East), state.if_coming_from(East)),
				(pos.if_coming_from(South), state.if_coming_from(South)),
				(pos.if_coming_from(West), state.if_coming_from(West)),
			]
			.into_iter()
			.filter(move |(next_pos, next_state)| {
				next_state.coming_from != (moving_backwards)
					&& (can_turn || is_start || next_state.coming_from == moving_forwards)
					&& (!has_to_turn || next_state.coming_from != moving_forwards)
					&& gridref.is_valid_pos(next_pos)
			})
			.map(|(pos, state)| {
				let cost = gridref.get_cost_at(&pos);
				((pos, state), cost)
			})
		},
		|(pos, _)| pos.distance(&goal),
		|(pos, state)| *pos == goal && state.straight_count >= 4,
	)
	.expect("must have shortest path");

	Solution(lowest_cost as usize, lowest_cost_ultra_crucible as usize)
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
