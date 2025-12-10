use aoc_utils::{ascii_int::parse_uint_coerce, trim::trim_end_newline};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

pub fn solve(input: &[u8]) -> Solution {
	let input = trim_end_newline(input);
	let last_line_start = input
		.iter()
		.rposition(|&c| c == b'\n')
		.expect("has multiple lines")
		+ 1;

	let op_line = &input[last_line_start..];
	let line_len = op_line.len() + 1; // added newline
	let line_amt = input[..last_line_start].len() / line_len;

	// for binary split comfort we split after the next operator, and start by reading the first.
	let mut op: u8 = op_line[0];
	let mut slice_cursor: usize = 0;

	let mut wrong_sum: usize = 0;
	let mut actual_sum: usize = 0;

	for op_str in op_line[1..].split_inclusive(|&c| c == b'*' || c == b'+') {
		let is_last = slice_cursor + op_str.len() + 1 == op_line.len();
		let len = op_str.len();
		let width = if is_last { len + 1 } else { len - 1 };

		let wrong_value_iter = (0..line_amt).map(|i| {
			let line_start = i * line_len;
			let start = line_start + slice_cursor;
			let end = start + width;
			parse_uint_coerce(&input[start..end])
		});

		let actual_value_iter = (0..width).map(|dig_i| {
			let mut value: usize = 0;
			for line_i in 0..line_amt {
				let line_start = line_i * line_len;
				let index = line_start + slice_cursor + dig_i;
				let byte = input[index];
				if byte >= b'0' {
					value *= 10;
					value += (byte - b'0') as usize;
				}
			}
			value
		});

		if op == b'+' {
			wrong_sum += wrong_value_iter.sum::<usize>();
			actual_sum += actual_value_iter.sum::<usize>();
		} else {
			wrong_sum += wrong_value_iter.product::<usize>();
			actual_sum += actual_value_iter.product::<usize>();
		}

		op = op_str[len - 1];
		slice_cursor += len;
	}

	Solution(wrong_sum, actual_sum)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(4277556, 3263827),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(4722948564882,9581313737063),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
