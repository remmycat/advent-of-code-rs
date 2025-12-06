use aoc_utils::{ascii_int::parse_uint_unchecked, trim::trim_end_newline};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

fn get_bank_joltage<const N: usize>(bank: &[u8], width: usize) -> usize {
	debug_assert!(width >= N, "Bank must have more than N items");

	let mut bank_start: usize = 0;
	let mut joltage_digits: [u8; N] = [0; N];

	for digit in 1..=N {
		let leftover_space = N - digit;
		let (max_l_pos, &max_l) = bank[bank_start..(width - leftover_space)]
			.iter()
			.enumerate()
			.rev() // get index of left-most from max_by_key
			.max_by_key(|p| p.1)
			.expect("Bank must have more than N items, see debug assertion");

		joltage_digits[digit - 1] = max_l;
		bank_start += max_l_pos + 1;
	}

	parse_uint_unchecked(&joltage_digits)
}

pub fn solve(input: &[u8]) -> Solution {
	let input = trim_end_newline(input);
	let bank_width = input
		.iter()
		.position(|b| *b == b'\n')
		.expect("input has multiple lines");

	let bank_amt = input.len().div_ceil(bank_width + 1);

	let mut small_joltage_sum = 0;
	let mut big_joltage_sum = 0;

	for bank_idx in 0..bank_amt {
		let in_l = bank_idx * (bank_width + 1);
		let in_r = in_l + bank_width;
		let bank = &input[in_l..in_r];

		// no relevant shared work i can find here
		small_joltage_sum += get_bank_joltage::<2>(bank, bank_width);
		big_joltage_sum += get_bank_joltage::<12>(bank, bank_width);
	}

	Solution(small_joltage_sum, big_joltage_sum)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(357,3121910778619),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(17109,169347417057382),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
