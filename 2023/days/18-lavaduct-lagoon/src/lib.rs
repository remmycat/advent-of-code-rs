use aoc_utils::{
	ascii_int::{parse_uint_hex_lowercase_unchecked, parse_uint_unchecked},
	iteration::expect_n,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

pub fn solve(input: &[u8]) -> Solution {
	// x mostly cancels out in shoelace sum, it's irrelevant to track

	let mut y: isize = 0;
	let mut shoelace_sum: isize = 0;
	let mut points_amount: usize = 0;

	let mut y_big: isize = 0;
	let mut shoelace_sum_big: isize = 0;
	let mut points_amount_big: usize = 0;

	for line in input.split(|b| *b == b'\n') {
		if line.is_empty() {
			continue;
		}

		let dir = line[0];
		let [amount, hex] = expect_n(line[2..].split(|b| *b == b' '), "expected amount");
		let amount = parse_uint_unchecked(amount);

		points_amount += amount;
		let amount = amount as isize;

		// generalisations of shoelace sums for <amount> increments in one direction
		match dir {
			// Up
			// Shoelace x part always evaluates to 0, so the product is 0
			b'U' => {
				// can be ignored for shoelace sum!
				y -= amount;
			}
			// Down
			// Shoelace x part always evaluates to 0, so the product is 0
			b'D' => {
				// can be ignored for shoelace sum!
				y += amount;
			}
			// Left
			// shoelace sum with diff (-1, 0) per point
			// (xn - xn1) part always evulates to -1
			// (yn + yn1) part always evaluates to y + y
			b'L' => {
				shoelace_sum += 2 * y * amount;
			}
			// Right
			// shoelace sum with diff (1, 0) per point
			// (xn - xn1) part always evulates to 1
			// (yn + yn1) part always evaluates to y + y
			b'R' => {
				shoelace_sum -= 2 * y * amount;
			}
			_ => panic!("unexpected dir"),
		};

		let dir = hex[7];
		let amount = parse_uint_hex_lowercase_unchecked(&hex[2..7]);

		points_amount_big += amount;

		let amount = amount as isize;

		match dir {
			// Up
			b'3' => {
				y_big -= amount;
			}
			// Down
			b'1' => {
				y_big += amount;
			}
			// Left
			b'2' => {
				shoelace_sum_big += 2 * y_big * amount;
			}
			// Right
			b'0' => {
				shoelace_sum_big -= 2 * y_big * amount;
			}
			_ => panic!("unexpected dir"),
		};
	}

	// shoelace wraparound will always evaluate to 0 due to start at 0,0!
	// so it's enough to add up all the determinants of x0..xN without the additional (xN, x0)

	let area = shoelace_sum.unsigned_abs() / 2;
	let points_inside = area - points_amount / 2 + 1;
	let points_total = points_inside + points_amount;

	let area_big = shoelace_sum_big.unsigned_abs() / 2;
	let points_inside_big = area_big - points_amount_big / 2 + 1;
	let points_total_big = points_inside_big + points_amount_big;

	Solution(points_total, points_total_big)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(62,952408144115),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(40745,90111113594927),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
