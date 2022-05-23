#![allow(clippy::many_single_char_names)]
use anyhow::Error;
use hashbrown::HashSet;
use std::str::FromStr;

pub struct Solution {
	straight_intersections: usize,
	all_intersections: usize,
}

#[derive(Clone, Debug)]
struct Line {
	start: (u16, u16),
	end: (u16, u16),
	is_straight: bool,
	direction: (i8, i8),
}

const X_SIZE: u16 = 1000;

impl FromStr for Line {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<_> = s
			.split(" -> ")
			.map(|coord| {
				let coords: Vec<_> = coord
					.split(',')
					.map(|c| c.parse::<u16>().unwrap())
					.collect();
				assert_eq!(coords.len(), 2);

				(coords[0], coords[1])
			})
			.collect();

		assert_eq!(parts.len(), 2);

		let start = parts[0];
		let end = parts[1];

		let is_straight = start.0 == end.0 || start.1 == end.1;

		let x_direction: i8 = match end.0 - start.0 {
			0 => 0,
			i if i > 0 => 1,
			_ => -1,
		};
		let y_direction: i8 = match end.1 - start.1 {
			0 => 0,
			i if i > 0 => 1,
			_ => -1,
		};

		Ok(Line {
			start,
			end,
			is_straight,
			direction: (x_direction, y_direction),
		})
	}
}

impl IntoIterator for Line {
	type Item = u16;
	type IntoIter = LineIterator;

	fn into_iter(self) -> Self::IntoIter {
		let size = (self.end.0 as i32 - self.start.0 as i32)
			.abs()
			.max((self.end.1 as i32 - self.start.1 as i32).abs()) as usize
			+ 1;

		LineIterator {
			size,
			end: self.end,
			direction: self.direction,
		}
	}
}

struct LineIterator {
	size: usize,
	end: (u16, u16),
	direction: (i8, i8),
}

impl Iterator for LineIterator {
	type Item = u16;

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.size, Some(self.size))
	}

	fn next(&mut self) -> Option<Self::Item> {
		if self.size == 0 {
			None
		} else {
			let size_mult = self.size as u16 - 1;
			let x = self.end.0 - self.direction.0 as u16 * size_mult;
			let y = self.end.1 - self.direction.1 as u16 * size_mult;
			self.size -= 1;
			Some(y * X_SIZE + x)
		}
	}
}

pub fn solve(input: &str) -> Solution {
	let lines = input.lines().map(|l| Line::from_str(l).unwrap());

	debug_assert!(lines.clone().map(|l| l.start.0.max(l.end.0)).max().unwrap() < X_SIZE);

	let mut straight_points: HashSet<u16> = HashSet::new();

	let straight_intersections: HashSet<u16> = lines
		.clone()
		.filter(|line| line.is_straight)
		.flat_map(|line| line.into_iter())
		.filter(|point| straight_points.insert(*point))
		.collect();
	let straight_intersections = straight_intersections.len();

	let mut all_points: HashSet<u16> = HashSet::new();

	let all_intersections: HashSet<u16> = lines
		.flat_map(|line| line.into_iter())
		.filter(|point| all_points.insert(*point))
		.collect();

	let all_intersections = all_intersections.len();

	Solution {
		straight_intersections,
		all_intersections,
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
		let input = fs::read_to_string("assets/05.txt").unwrap();

		assert_eq!(solve(&input).straight_intersections, 6666);
	}

	#[test]
	fn part_2_example_cases() {
		let example = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

		assert_eq!(solve(example).all_intersections, 12);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/05.txt").unwrap();

		assert_eq!(solve(&input).all_intersections, 19081);
	}
}
