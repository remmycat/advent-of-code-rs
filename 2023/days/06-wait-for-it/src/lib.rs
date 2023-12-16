use aoc_utils::{
	ascii_int::{parse_uint, parse_uint_coerce},
	iteration::expect_n,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

const LINE_OFFSET: usize = "Distance: ".len();

fn calculate_win_possibilities((time, dist): (usize, usize)) -> usize {
	// we need to break, not match the record
	let target_distance = dist + 1;
	// solve quadratic equation to get min/max values for press time
	let root_part = ((time * time - 4 * target_distance) as f64).sqrt();
	let min_time = (time as f64 - root_part) / 2.0;
	let max_time = (time as f64 + root_part) / 2.0;

	// we need integer values. since we need to be between min and max, round min up, max down.
	let min_time = min_time.ceil() as usize;
	let max_time = max_time.floor() as usize;

	max_time - min_time + 1
}

pub fn solve(input: &[u8]) -> Solution {
	let [time_line, dist_line] = expect_n(input.split(|b| *b == b'\n'), "input has 2 lines");

	let times = time_line[LINE_OFFSET..]
		.split(|b| *b == b' ')
		.filter(|part| !part.is_empty())
		.map(parse_uint);
	let dists = dist_line[LINE_OFFSET..]
		.split(|b| *b == b' ')
		.filter(|part| !part.is_empty())
		.map(parse_uint);

	let multiplied_possibilities: usize =
		times.zip(dists).map(calculate_win_possibilities).product();

	let time_keming = parse_uint_coerce(&time_line[LINE_OFFSET..]);
	let dist_keming = parse_uint_coerce(&dist_line[LINE_OFFSET..]);

	let keming_possibilities = calculate_win_possibilities((time_keming, dist_keming));

	Solution(multiplied_possibilities, keming_possibilities)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(288,71503),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(5133600,40651271),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
