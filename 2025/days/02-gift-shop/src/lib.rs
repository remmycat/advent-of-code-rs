use aoc_utils::{ascii_int::parse_uint, trim::trim_end_newline};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

const COMMA: u8 = b',';
const DASH: u8 = b'-';
const BASE_TEN: usize = 10;

pub fn solve(input: &[u8]) -> Solution {
	let input = trim_end_newline(input);

	let pair_iter = input.split(|b| *b == COMMA).map(|range_str| {
		let dash_pos = range_str
			.iter()
			.position(|b| *b == DASH)
			.expect("range must have dash separator");
		let (a_str, b_str) = (
			&range_str[0..dash_pos],
			&range_str[(dash_pos + 1)..(range_str.len())],
		);
		(a_str, parse_uint(a_str), b_str, parse_uint(b_str))
	});

	let mut invalid_sum: usize = 0;
	let mut big_invalid_sum: usize = 0;

	let mut counted_set = BTreeSet::<usize>::new();

	for (a_str, a, b_str, b) in pair_iter {
		let (start_len, end_len) = (a_str.len(), b_str.len());

		// Possible split points: any integer between 2 and end length
		// example 3
		counted_set.clear();
		for repetitions in 2..=end_len {
			// we only care about numbers that divide by split_len without rest
			// example 12345-123456789
			// start_split_len = 2 (5 /+ 3 = 2)
			// end_split_len = 3 (9 /- 3 = 3)
			let start_split_len = start_len.div_ceil(repetitions);
			let end_split_len = (end_len) / repetitions;

			// e.g 2..=3
			for split_len in start_split_len..=end_split_len {
				// e.g. 6 this is fix inside this loop! only 6-digit numbers
				let str_len = repetitions * split_len;
				let first_split_len_digits_base = BASE_TEN.pow((str_len - split_len) as u32);
				//const
				let split_range_start = if str_len == start_len {
					// example if 123456 was our start we want 12 so / 10^(6-2)
					a / first_split_len_digits_base
				} else {
					// e.g. 10
					BASE_TEN.pow(split_len as u32 - 1)
				};
				let split_range_end = if str_len == end_len {
					// example if 123456 was our end we want 12 so / 10^(6-2)
					b / first_split_len_digits_base
				} else {
					// e.g 99
					BASE_TEN.pow(split_len as u32) - 1
				};
				if split_range_end < split_range_start {
					break;
				}
				// e.g 12 * 10101 = 121212 // 10^4 + 10^2 + 10^0 (3 reps)
				// or 123 * 1001 = 123123 // 10^3 + 10^0 (2 reps)
				let repetition_base: usize = (0..repetitions)
					.map(|k| BASE_TEN.pow((k * split_len) as u32))
					.sum();
				for split_id in split_range_start..=split_range_end {
					let full_id = split_id * repetition_base;
					if full_id <= b {
						if full_id >= a {
							if repetitions == 2 {
								invalid_sum += full_id;
							}
							if !counted_set.contains(&full_id) {
								big_invalid_sum += full_id;
								counted_set.insert(full_id);
							}
						}
						// else not reached valid range yet
					} else {
						// got bigger than valid range end
						break;
					}
				}
			}
		}
	}

	Solution(invalid_sum, big_invalid_sum)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(1227775554,4174379265),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(22062284697,46666175279),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
