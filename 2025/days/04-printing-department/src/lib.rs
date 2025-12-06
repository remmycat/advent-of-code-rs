use aoc_utils::trim::trim_end_newline;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

struct Roll {
	index: usize,
	neighbours: [Option<usize>; 8],
}

// copy of map, updating using removal

pub fn solve(input: &[u8]) -> Solution {
	let input = trim_end_newline(input);
	let width = input
		.iter()
		.position(|b| *b == b'\n')
		.expect("input has multiple lines");
	let row_width = width + 1;
	let size = input.len();
	let is_in_range = |x: &usize| *x < size;
	let _height = input.len().div_ceil(row_width);

	let mut leftovers: Vec<_> = input.to_vec();

	let mut rolls: Vec<Roll> = leftovers
		.iter()
		.enumerate()
		.filter(|(_, c)| **c == b'@')
		.map(|(index, _)| Roll {
			index,
			neighbours: [
				index.checked_sub(row_width + 1),
				index.checked_sub(row_width),
				index.checked_sub(row_width - 1),
				index.checked_sub(1),
				Some(index + 1).filter(is_in_range),
				Some(index + row_width - 1).filter(is_in_range),
				Some(index + row_width).filter(is_in_range),
				Some(index + row_width + 1).filter(is_in_range),
			],
		})
		.collect();

	let first_step_removed = rolls
		.iter()
		.filter(|roll| {
			roll.neighbours
				.iter()
				.flatten()
				.filter(|i| leftovers[**i] == b'@')
				.count() < 4
		})
		.count();

	let roll_count = rolls.len();

	let mut current_count = roll_count;

	loop {
		rolls.retain(|roll| {
			if roll
				.neighbours
				.iter()
				.flatten()
				.filter(|i| leftovers[**i] == b'@')
				.count() < 4
			{
				leftovers[roll.index] = 0;
				false
			} else {
				true
			}
		});
		let new_count = rolls.len();
		if new_count == current_count {
			break;
		}
		current_count = new_count;
	}

	let all_removed = roll_count - rolls.len();

	Solution(first_step_removed, all_removed)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(13,43),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(1424,8727),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
