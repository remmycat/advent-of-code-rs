use aoc_2023_utils::range_set::IntRangeSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug)]
struct Point {
	x: usize,
	y: usize,
	x_ex: usize,
	y_ex: usize,
}

fn parse_points(input: &[u8]) -> Vec<Point> {
	let width = input
		.iter()
		.position(|b| *b == b'\n')
		.expect("input must have newline");
	let line_width = width + 1;
	let height = input.len() / line_width;

	let mut x_expanses = IntRangeSet::<usize>::new();
	let mut y_expanses = IntRangeSet::<usize>::new();

	let mut points: Vec<_> = input
		.iter()
		.enumerate()
		.filter(|(_, b)| **b == b'#')
		.map(|(index, _)| {
			let x = index % line_width;
			let y = index / line_width;

			x_expanses.add_one(x);
			y_expanses.add_one(y);

			Point {
				x,
				y,
				x_ex: 0,
				y_ex: 0,
			}
		})
		.collect();

	x_expanses.invert((0, width - 1));
	y_expanses.invert((0, height - 1));

	points.sort_unstable_by_key(|point| point.x);

	let mut point_iter = points.iter_mut().peekable();
	let mut expansion = 0;
	for range in x_expanses.ranges.into_iter() {
		while let Some(point) = point_iter.next_if(|point| point.x < range.0) {
			point.x_ex = expansion;
		}
		expansion += range.1 + 1 - range.0
	}
	for point in point_iter {
		point.x_ex += expansion;
	}

	// expand y
	points.sort_unstable_by_key(|point| point.y);
	let mut point_iter = points.iter_mut().peekable();
	let mut expansion = 0;
	for range in y_expanses.ranges.into_iter() {
		while let Some(point) = point_iter.next_if(|point| point.y < range.0) {
			point.y_ex = expansion;
		}
		expansion += range.1 + 1 - range.0
	}
	for point in point_iter {
		point.y_ex = expansion;
	}

	points
}

pub fn solve(input: &[u8]) -> Solution {
	let points = parse_points(input);

	let mut points_distance = 0;
	let mut expansion_distance = 0;

	let mut points_iter = points.iter();

	while let Some(a) = points_iter.next() {
		for b in points_iter.clone() {
			points_distance += a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
			expansion_distance += a.x_ex.abs_diff(b.x_ex) + a.y_ex.abs_diff(b.y_ex)
		}
	}

	let points_distance_new_galaxies = points_distance + expansion_distance;
	let points_distance_old_galaxies = points_distance + expansion_distance * 999_999;

	Solution(points_distance_new_galaxies, points_distance_old_galaxies)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(include_bytes!("../inputs/example.txt"), Solution(374,82000210))]
	#[case::personal(include_bytes!("../inputs/personal.txt"), Solution(9918828,692506533832))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
