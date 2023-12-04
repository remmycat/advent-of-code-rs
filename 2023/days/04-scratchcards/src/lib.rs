#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

const EMPTY: [bool; 100] = [false; 100];

fn parse_2_digit_uint(input: &[u8]) -> usize {
	// println!("{}", String::from_utf8(input.to_vec()).unwrap());
	(if input[0] == b' ' {
		input[1] - b'0'
	} else {
		(input[0] - b'0') * 10 + (input[1] - b'0')
	}) as usize
}

pub fn solve(input: &[u8]) -> Solution {
	let mut copies_won: [usize; 500] = [0; 500];
	let mut card_numbers: [bool; 100];
	let mut points_won = 0;

	let input_width = input
		.iter()
		.position(|b| *b == b'\n')
		.expect("input must have newline")
		+ 1;
	let line_prefix_width = input
		.iter()
		.position(|b| *b == b':')
		.expect("input must have colon")
		+ 2;
	let pipe_position = input
		.iter()
		.position(|b| *b == b'|')
		.expect("input must have pipe");
	let after_pipe = pipe_position + 2;
	let win_nrs = (pipe_position - line_prefix_width) / 3;
	let card_nrs = (input_width - after_pipe) / 3;

	for (index, line) in input.chunks(input_width).enumerate() {
		let game = index + 1;
		card_numbers = EMPTY;

		let game_copies = copies_won[game] + 1;
		copies_won[game] = game_copies;

		for win_index in 0..win_nrs {
			let span_start = line_prefix_width + 3 * win_index;
			let span_end = span_start + 2;
			let win_nr = parse_2_digit_uint(&line[span_start..span_end]);
			card_numbers[win_nr] = true;
		}

		let winning = (0..card_nrs)
			.map(|idx| {
				let span_start = after_pipe + 3 * idx;
				let span_end = span_start + 2;
				parse_2_digit_uint(&line[span_start..span_end])
			})
			.filter(|num| card_numbers[*num])
			.count();

		// manual power of 2
		points_won += 1_usize << winning >> 1;

		for won_copy in (game + 1..).take(winning) {
			copies_won[won_copy] += game_copies
		}
	}

	let copies_won = copies_won.iter().sum();

	Solution(points_won, copies_won)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(13,30))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(25183,5667240))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
