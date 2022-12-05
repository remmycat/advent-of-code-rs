#[derive(Debug, PartialEq, Eq)]
pub struct Solution(String, String);

const SPLIT: u8 = b'\n';
const SPACE: u8 = b' ';
const NUMBERS_START: u8 = b'0';

pub fn solve_bytes(input: &[u8]) -> Solution {
	let mut stacks_9000: [Vec<u8>; 9] = Default::default();

	let mut line_iter = input.split(|b| *b == SPLIT);

	let mut init_lines: Vec<_> = (&mut line_iter)
		.take_while(|line| !line.is_empty())
		.collect();

	// remove useless numbers
	init_lines.pop();
	// start at bottom of stacks for easier parsing
	init_lines.reverse();

	for line in init_lines {
		for (index, id) in line
			.chunks(4)
			.map(|chunk| chunk.get(1).expect("malformed input, expected crate id"))
			.enumerate()
			.filter(|(_, id)| (**id as char).is_ascii_alphabetic())
		{
			stacks_9000[index].push(*id);
		}
	}

	let mut stacks_9001 = stacks_9000.clone();

	for line in line_iter {
		if line.is_empty() {
			break;
		}

		let mut parts = line.split(|b| *b == SPACE);

		let amt = parts
			.nth(1)
			.and_then(|amt| std::str::from_utf8(amt).ok())
			.and_then(|num_str| num_str.parse::<usize>().ok())
			.expect("malformed input, expected crate amount");

		let from = parts
			.nth(1)
			.and_then(|from| from.first())
			.map(|from| (*from - NUMBERS_START) as usize)
			.expect("malformed input, expected 'from' value (stack index)");

		let to = parts
			.nth(1)
			.and_then(|to| to.first())
			.map(|to| (*to - NUMBERS_START) as usize)
			.expect("malformed input, expected 'to' value (stack index)");

		for _ in 0..amt {
			let id = stacks_9000[from - 1].pop().expect("bad instructions?");
			stacks_9000[to - 1].push(id);
		}

		let new_size = stacks_9001[from - 1].len() - amt;
		let crate_ids: Vec<_> = stacks_9001[from - 1].drain(new_size..).collect();
		stacks_9001[to - 1].extend(crate_ids);
	}

	let tops_9000: String = stacks_9000
		.iter_mut()
		.filter_map(|s| s.pop())
		.map(|id| id as char)
		.collect();

	let tops_9001: String = stacks_9001
		.iter_mut()
		.filter_map(|s| s.pop())
		.map(|id| id as char)
		.collect();

	Solution(tops_9000, tops_9001)
}

pub fn solve_string(input: &str) -> Solution {
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
	#[case(include_bytes!("../inputs/example.txt"), Solution("CMZ".into(),"MCD".into()))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution("DHBJQJCCW".into(), "WJVRLSJJT".into()))]
	fn solution_bytes(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve_bytes(input), expected);
	}

	#[rstest]
	#[case(include_str!("../inputs/example.txt"), Solution("CMZ".into(),"MCD".into()))]
	#[case(include_str!("../inputs/personal.txt"), Solution("DHBJQJCCW".into(), "WJVRLSJJT".into()))]
	fn solution_string(#[case] input: &str, #[case] expected: Solution) {
		assert_eq!(solve_string(input), expected);
	}
}
