#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

const EMPTY: [bool; 100] = [false; 100];

fn parse_2_digit_uint(input: &[u8]) -> usize {
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

	for (index, line) in input
		.split(|b| *b == b'\n')
		.filter(|line| !line.is_empty())
		.enumerate()
	{
		let game = index + 1;
		card_numbers = EMPTY;

		let game_copies = copies_won[game] + 1;
		copies_won[game] = game_copies;

		let line = line
			.split(|b| *b == b':')
			.nth(1)
			.expect("bad input, expected data after :");

		let mut parts = line.split(|b| *b == b'|');
		let (wins, nums) = (
			&parts.next().expect("bad input, expected win numbers")[1..],
			&parts.next().expect("bad input, expected card numbers")[1..],
		);

		for win_nr in wins.chunks(3).map(parse_2_digit_uint) {
			card_numbers[win_nr] = true;
		}

		let winning = nums
			.chunks(3)
			.map(parse_2_digit_uint)
			.filter(|num| card_numbers[*num])
			.count();

		if winning > 0 {
			points_won += 2_usize.pow((winning - 1) as u32);
		}

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
