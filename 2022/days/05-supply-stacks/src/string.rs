use crate::Solution;

pub fn solve(input: &str) -> Solution {
	let mut stacks_9000: Vec<Vec<char>> = vec![vec![]; 9];

	let (init, moves) = input
		.split_once("\n\n")
		.expect("malformed input, expected 2 parts");

	for line in init.lines().rev() {
		line.chars()
			.enumerate()
			.filter_map(|(index, char)| if index % 4 == 1 { Some(char) } else { None })
			.enumerate()
			.filter(|(_, c)| c.is_ascii_alphabetic())
			.for_each(|(stack, crate_id)| stacks_9000[stack].push(crate_id));
	}

	let mut stacks_9001 = stacks_9000.clone();

	for line in moves.trim().lines() {
		let parts: Vec<_> = line.split(' ').collect();

		let amt = parts
			.get(1)
			.and_then(|num_str| num_str.parse::<usize>().ok())
			.expect("malformed input, expected crate amount");

		let from = parts
			.get(3)
			.and_then(|from| from.parse::<usize>().ok())
			.expect("malformed input, expected from (stack index)");

		let to = parts
			.get(5)
			.and_then(|to| to.parse::<usize>().ok())
			.expect("malformed input, expected to (stack index)");

		// CrateMover 9000 - crates get reversed
		let new_from_size = stacks_9000[from - 1].len() - amt;
		let mut crate_ids: Vec<_> = stacks_9000[from - 1].split_off(new_from_size);
		crate_ids.reverse();
		stacks_9000[to - 1].append(&mut crate_ids);

		// CrateMover 9001 - crates do not get reversed
		let new_from_size = stacks_9001[from - 1].len() - amt;
		let mut crate_ids: Vec<_> = stacks_9001[from - 1].split_off(new_from_size);
		stacks_9001[to - 1].append(&mut crate_ids);
	}

	let tops_9000: String = stacks_9000.iter_mut().filter_map(|s| s.pop()).collect();

	let tops_9001: String = stacks_9001.iter_mut().filter_map(|s| s.pop()).collect();

	Solution(tops_9000, tops_9001)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_str!("../inputs/example.txt"), Solution("CMZ".into(),"MCD".into()))]
	#[case(include_str!("../inputs/personal.txt"), Solution("DHBJQJCCW".into(), "WJVRLSJJT".into()))]
	fn solution(#[case] input: &str, #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
