use aoc_utils::{ascii_int::parse_uint_unchecked, trim::trim_end_newline};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

pub fn solve(input: &[u8]) -> Solution {
	let input = trim_end_newline(input);
	let space_width = 3;
	let num_width = input
		.iter()
		.position(|b| *b == b' ')
		.expect("must have space");
	let rnum_offset = num_width + space_width;
	let line_width = 2 * num_width + space_width + 1;

	let (mut left, mut right): (Vec<usize>, Vec<usize>) = input
		.chunks(line_width)
		.map(|line| {
			let num_l = parse_uint_unchecked(&line[0..num_width]);
			let num_r = parse_uint_unchecked(&line[rnum_offset..(rnum_offset + num_width)]);

			(num_l, num_r)
		})
		.collect();

	left.sort_unstable();
	right.sort_unstable();

	let difference: usize = left.iter().zip(&right).map(|(a, b)| a.abs_diff(*b)).sum();

	let right_run_length: Vec<_> = right
		.chunk_by(|a, b| a == b)
		.map(|chunk| (chunk[0], chunk.len()))
		.collect();

	let similarity_score: usize = left
		.chunk_by(|a, b| a == b)
		.map(|chunk| {
			let left_amt = chunk.len();
			let id = &chunk[0];
			let right_amt = right_run_length
				.binary_search_by_key(id, |pair| pair.0)
				.map(|idx| right_run_length[idx].1)
				.unwrap_or(0);

			*id * left_amt * right_amt
		})
		.sum();

	Solution(difference, similarity_score)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(11,31),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(1941353,22539317),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
