use aoc_utils::ascii_int::parse_int;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(isize, isize);

fn save_diffs(firsts: &mut Vec<isize>, diffs: &mut Vec<isize>, comp: isize) -> bool {
	let mut next = comp;
	for latest in diffs.iter_mut() {
		if *latest == 0 && next == 0 {
			return true;
		}

		let diff = next - *latest;
		*latest = next;
		next = diff;
	}

	firsts.push(next);
	diffs.push(next);

	false
}

fn extrapolate_once(diffs: &mut [isize]) {
	let mut last_res = 0;
	for latest in diffs.iter_mut().rev().skip(1) {
		*latest += last_res;
		last_res = *latest;
	}
}

fn extrapolate_from_line(
	shared_firsts: &mut Vec<isize>,
	shared_diffs: &mut Vec<isize>,
	line: &[u8],
) -> (isize, isize) {
	shared_firsts.clear();
	shared_diffs.clear();

	let mut can_extrapolate = false;

	for part in line.split(|b| *b == b' ') {
		// we can stop parsing input once the first 0 0 diffs were found
		// instead just extrapolate once for every further input span found
		if can_extrapolate {
			extrapolate_once(shared_diffs);
		} else {
			can_extrapolate = save_diffs(shared_firsts, shared_diffs, parse_int(part));
		}
	}

	let next = shared_diffs.iter().sum();
	let prev = shared_firsts
		.iter()
		.rev()
		.cloned()
		.reduce(|acc, el| el - acc)
		.expect("line must have at least one element");

	(prev, next)
}

pub fn solve(input: &[u8]) -> Solution {
	let mut shared_firsts = vec![];
	let mut shared_diffs = vec![];
	let mut extrapolated_prev = 0;
	let mut extrapoled_next = 0;

	for (prev, next) in input
		.split(|b| *b == b'\n')
		.filter(|l| !l.is_empty())
		.map(|line| extrapolate_from_line(&mut shared_firsts, &mut shared_diffs, line))
	{
		extrapolated_prev += prev;
		extrapoled_next += next;
	}

	Solution(extrapoled_next, extrapolated_prev)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(114,2),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(1681758908,803),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
