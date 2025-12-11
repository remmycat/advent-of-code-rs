use aoc_utils::trim::trim_end_newline;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

pub fn solve(input: &[u8]) -> Solution {
	let input = trim_end_newline(input);
	let line_width = input
		.iter()
		.position(|&c| c == b'\n')
		.expect("input has multiple lines");
	let start = input[0..line_width]
		.iter()
		.position(|&c| c == b'S')
		.expect("first line has S");

	let height = input.len().div_ceil(line_width + 1);

	let mut beams = vec![0_usize; line_width];
	beams[start] = 1;

	let mut splits = 0;

	// skip every second line for calculation, it's just dots
	for (i, line) in (1..height / 2).map(|i| {
		let line_start = i * 2 * (line_width + 1);
		(i, &input[line_start..(line_start + line_width)])
	}) {
		let min_index = start - i; // we can grow one to the left in every row…
		let max_length = 2 * i + 1; // …and one to the right
		for (split_pos, _) in line
			.iter()
			.enumerate()
			.skip(min_index)
			.take(max_length)
			.filter(|&(_, &c)| c == b'^')
		{
			if beams[split_pos] != 0 {
				let previous = beams[split_pos];
				// splitters seem to be padded between each other and the border
				beams[split_pos] = 0;
				// we get 2 times as many timelines for all beams that were in
				// this position.
				beams[split_pos - 1] += previous;
				beams[split_pos + 1] += previous;
				splits += 1;
			}
		}
	}

	let possible_timelines = beams.iter().sum::<usize>();

	Solution(splits, possible_timelines)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(21,40),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(1535,4404709551015),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
