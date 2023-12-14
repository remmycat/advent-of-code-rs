use std::cmp::Ordering;
use std::fmt;

use aoc_2023_utils::{ascii_int::parse_uint_unchecked, iteration::expect_n};
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Hash, PartialEq, Eq, Clone)]
struct PicrossState {
	// bitmap 0 = not filled or unknown, 1 = filled
	filled: u128,
	// bitmap 0 = known (see filled), 1 = unknown
	unknown: u128,
	// coun
	counts: u128,
}

impl fmt::Debug for PicrossState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let PicrossState {
			counts,
			filled,
			unknown,
		} = self;

		write!(
			f,
			"PicrossState\n    counts: {counts:0128b}\n    filled: {filled:0100b}\n    unknown: {unknown:0100b}"
		)
	}
}

impl PicrossState {
	fn parse_from_line_small_big(line: &[u8]) -> (PicrossState, PicrossState) {
		let [pixels, counts] = expect_n(line.split(|b| *b == b' '), "every line has a space");

		// expects max 20 pixels
		let (filled, unknown, fill_len) = pixels
			.iter()
			.enumerate()
			.map(|(i, pixel)| match pixel {
				b'.' => (0, 0, 1),
				b'#' => (1 << i, 0, 1),
				b'?' => (0, 1 << i, 1),
				_ => panic!("Unexpected byte in pixels"),
			})
			.fold(
				(0_u128, 0_u128, 0_u32),
				|(acc_fill, acc_unknown, acc_len), (fill, unknown, len)| {
					(acc_fill | fill, acc_unknown | unknown, acc_len + len)
				},
			);

		// This barely fits, but should always work for 20 pixels, with max count 16!
		let (counts, counts_len) = counts
			.split(|b| *b == b',')
			.rev()
			.map(parse_uint_unchecked)
			.fold((0_u128, 0_u32), |(acc, len), count| match count.cmp(&16) {
				Ordering::Less => (acc << 4 | (count as u128), len + 4),
				Ordering::Equal => (acc << 5 | (count as u128), len + 5),
				Ordering::Greater => panic!("count bigger than 16"),
			});

		let small_state = PicrossState {
			counts,
			filled,
			unknown,
		};

		let mut big_state = small_state.clone();

		for _ in 0..4 {
			big_state.filled = big_state.filled << (fill_len + 1) | small_state.filled;
			big_state.unknown = ((big_state.unknown << 1 | 1) << fill_len) | small_state.unknown;
			big_state.counts =
				big_state.counts.checked_shl(counts_len).expect("fits") | small_state.counts;
		}

		// println!("SMALL: {small_state:?}\nBIG: {big_state:?}");

		(small_state, big_state)
	}
}

// oh no, it's a hashmap-or-lose day
type PicrossArrangements = HashMap<PicrossState, usize>;

fn ends_in_one(num: u128) -> bool {
	num & 1 == 1
}

fn sum_up_permutations(mut state: PicrossState, memo: &mut PicrossArrangements) -> usize {
	let mut sum = 0;

	loop {
		let possibilities = state.filled | state.unknown;
		if possibilities == 0 {
			break;
		}

		// CHECK IF LAST BIT of filled or unknown IS 1
		if ends_in_one(possibilities) {
			sum += if let Some(memoized) = memo.get(&state) {
				*memoized
			} else {
				let result = check_permutation(state.clone(), memo);
				memo.insert(state.clone(), result);
				result
			};
		}

		// CHECK IF LAST BIT of filled IS 1
		// we cannot continue, because this pixel must be covered by a hint!
		if ends_in_one(state.filled) {
			break;
		}

		state.filled >>= 1;
		state.unknown >>= 1;
	}

	// println!("PERMUTATION SET END: {sum}\n");

	sum
}

const COUNT_BITS: u128 = 0b1111;

fn check_permutation(mut state: PicrossState, memo: &mut PicrossArrangements) -> usize {
	let count = state.counts & COUNT_BITS;
	state.counts >>= 4;

	// special case 16 encoding
	let count = if count == 0 && state.counts != 0 {
		state.counts >>= 1;
		16
	} else {
		count
	};

	let is_end_fill = state.counts == 0;

	let possible_fill = state.filled | state.unknown;

	let maybe_fitting = possible_fill | (u128::MAX << count);

	let fits = maybe_fitting == u128::MAX && {
		// COUNT fits
		state.filled >>= count;
		state.unknown >>= count;

		if is_end_fill {
			true
		} else {
			// CHECK IF LAST BIT IS SET
			let fits_gap = !ends_in_one(state.filled);
			// println!("fits gap: {}", fits_gap);
			state.filled >>= 1;
			state.unknown >>= 1;
			fits_gap
		}
	};

	if !fits {
		0
	} else if is_end_fill {
		if state.filled == 0 {
			// satisfied all counts
			1
		} else {
			// used up all counts, but no space left
			0
		}
	} else {
		sum_up_permutations(state, memo)
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let mut memo: PicrossArrangements = HashMap::new();

	let mut small_sum = 0;
	let mut huge_sum = 0;

	for (small_state, huge_state) in input
		.split(|b| *b == b'\n')
		.filter(|l| !l.is_empty())
		.map(PicrossState::parse_from_line_small_big)
	{
		small_sum += sum_up_permutations(small_state, &mut memo);
		huge_sum += sum_up_permutations(huge_state, &mut memo);
	}

	Solution(small_sum, huge_sum)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(include_bytes!("../inputs/example.txt"), Solution(21,525152))]
	#[case::personal(include_bytes!("../inputs/personal.txt"), Solution(8193,45322533163795))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}

	#[rstest]
	#[case("???.### 1,1,3", (1,1))]
	#[case(".??..??...?##. 1,1,3", (4, 16384))]
	#[case("?#?#?#?#?#?#?#? 1,3,1,6", (1,1))]
	#[case("????.#...#... 4,1,1", (1, 16))]
	#[case("????.######..#####. 1,6,5", (4, 2500))]
	#[case("?###???????? 3,2,1", (10, 506250))]
	fn count_arrangements_examples(#[case] input: &str, #[case] expected: (usize, usize)) {
		let mut memo: PicrossArrangements = HashMap::new();
		let (small, big) = PicrossState::parse_from_line_small_big(input.as_bytes());
		assert_eq!(
			(
				sum_up_permutations(small, &mut memo),
				sum_up_permutations(big, &mut memo)
			),
			expected
		);
	}
}
