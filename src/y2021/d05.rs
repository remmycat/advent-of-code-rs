#![allow(clippy::many_single_char_names)]
use anyhow::Error;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Solution {
	straight_intersections: usize,
	all_intersections: usize,
}

#[derive(Clone, PartialEq, Debug)]
enum LineDirection {
	Horizontal,
	Vertical,
	Other,
}

#[derive(Clone, Debug)]
struct Line {
	start: (i32, i32),
	end: (i32, i32),
	direction: LineDirection,
}

impl FromStr for Line {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<_> = s
			.split(" -> ")
			.map(|coord| {
				let coords: Vec<_> = coord
					.split(',')
					.map(|c| c.parse::<i32>().unwrap())
					.collect();
				assert_eq!(coords.len(), 2);

				(coords[0], coords[1])
			})
			.collect();

		assert_eq!(parts.len(), 2);

		let start = parts[0];
		let end = parts[1];

		let direction = match (start, end) {
			((x1, _), (x2, _)) if x1 == x2 => LineDirection::Vertical,
			((_, y1), (_, y2)) if y1 == y2 => LineDirection::Horizontal,
			_ => LineDirection::Other,
		};

		Ok(Line {
			start,
			end,
			direction,
		})
	}
}

impl Line {
	fn is_straight(&self) -> bool {
		self.direction == LineDirection::Horizontal || self.direction == LineDirection::Vertical
	}
}

struct Map {
	straight_intersections: HashSet<(i32, i32)>,
	all_intersections: HashSet<(i32, i32)>,
	straight_points: HashSet<(i32, i32)>,
	all_points: HashSet<(i32, i32)>,
}

impl Map {
	fn new() -> Self {
		Map {
			straight_points: HashSet::new(),
			all_points: HashSet::new(),
			straight_intersections: HashSet::new(),
			all_intersections: HashSet::new(),
		}
	}

	fn draw_point(&mut self, coords: (i32, i32), is_straight: bool) {
		if is_straight {
			if self.straight_points.contains(&coords) {
				self.straight_intersections.insert(coords);
			} else {
				self.straight_points.insert(coords);
			}
		}
		if self.all_points.contains(&coords) {
			self.all_intersections.insert(coords);
		} else {
			self.all_points.insert(coords);
		}
	}

	fn draw_line(&mut self, line: Line) {
		let x_direction: i32 = match line.end.0 - line.start.0 {
			0 => 0,
			i if i > 0 => 1,
			_ => -1,
		};
		let y_direction: i32 = match line.end.1 - line.start.1 {
			0 => 0,
			i if i > 0 => 1,
			_ => -1,
		};

		let mut current = line.start.to_owned();

		let is_straight = line.is_straight();

		loop {
			self.draw_point(current, is_straight);
			if current.0 == line.end.0 && current.1 == line.end.1 {
				break;
			}

			current.0 += x_direction;
			current.1 += y_direction;
		}
	}
}

pub fn solve(input: &str) -> Solution {
	let mut map = Map::new();

	input
		.lines()
		.map(|input_line| input_line.parse::<Line>().unwrap())
		.for_each(|line| map.draw_line(line));

	Solution {
		straight_intersections: map.straight_intersections.len(),
		all_intersections: map.all_intersections.len(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

		assert_eq!(solve(example).straight_intersections, 5);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_05.txt").unwrap();

		assert_eq!(solve(&input).straight_intersections, 6666);
	}

	#[test]
	fn part_2_example_cases() {
		let example = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

		assert_eq!(solve(example).all_intersections, 12);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_05.txt").unwrap();

		assert_eq!(solve(&input).all_intersections, 19081);
	}
}
