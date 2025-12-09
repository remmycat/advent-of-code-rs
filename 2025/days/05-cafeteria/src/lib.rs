use aoc_utils::{ascii_int::parse_uint, range_set::IntRangeSet, trim::trim_end_newline};

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

	let mut range_set: IntRangeSet<usize> = IntRangeSet::new();

	for range in range_input.split(|&c| c == b'\n').map(|range_str| {
		let (left, right) = range_str.split_at(
			range_str
				.iter()
				.position(|&c| c == b'-')
				.expect("range must have dash"),
		);
		let right = &right[1..];
		(parse_uint(left), parse_uint(right))
	}) {
		range_set.add_range(range);
	}

	let included = item_input
		.split(|&c| c == b'\n')
		.map(parse_uint)
		.filter(|item| range_set.ranges.iter().any(|(a, b)| item >= a && item <= b))
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
