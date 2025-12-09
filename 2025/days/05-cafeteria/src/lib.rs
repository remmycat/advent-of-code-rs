use aoc_utils::{range_set::IntRangeSet, trim::trim_end_newline};
use atoi_simd as atoi;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

pub fn solve(input: &[u8]) -> Solution {
	let input = trim_end_newline(input);
	let last_dash_pos = input
		.iter()
		.rposition(|&c| c == b'-')
		.expect("must have one dash in valid input");
	let rel_break_pos = input[last_dash_pos..]
		.iter()
		.position(|c| *c == b'\n')
		.expect("must have newline after last range");
	let break_pos = last_dash_pos + rel_break_pos;

	let range_input = &input[..break_pos];
	let item_input = &input[(break_pos + 2)..];

	let range_set: IntRangeSet<usize> = range_input
		.split(|&c| c == b'\n')
		.map(|range_str| {
			let (left, right) = range_str.split_at(
				range_str
					.iter()
					.position(|&c| c == b'-')
					.expect("range must have dash"),
			);
			let right = &right[1..];
			(
				atoi::parse_pos::<usize>(left).expect("valid int"),
				atoi::parse_pos::<usize>(right).expect("valid int"),
			)
		})
		.fold(IntRangeSet::new(), |mut set, range| {
			set.add_range(range);
			set
		});

	let included = item_input
		.split(|&c| c == b'\n')
		.map(|item| atoi::parse_pos::<usize>(item).expect("valid uint"))
		.filter(|item| range_set.contains(item))
		.count();

	Solution(included, range_set.len())
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(3,14),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(558,344813017450467),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
