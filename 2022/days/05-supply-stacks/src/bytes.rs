use core::slice;

use crate::Solution;

const NUMBERS_START: u8 = b'0';

fn get_peaks(stacks: &[Vec<u8>]) -> String {
	stacks
		.iter()
		.filter_map(|s| s.last())
		.map(|id| *id as char)
		.collect()
}

fn instruction_9000(stacks: &mut [Vec<u8>; 9], amt: usize, from: usize, to: usize) {
	for _ in 0..amt {
		let id = stacks[from].pop().expect("bad instructions?");
		stacks[to].push(id);
	}
}

fn instruction_9001(stacks: &mut [Vec<u8>; 9], amt: usize, from: usize, to: usize) {
	assert_ne!(from, to);
	let from_stack_ptr = stacks[from].as_ptr();
	let new_size = stacks[from].len() - amt;

	// Since we know from and to are different stacks, circumventing the borrow
	// checker lets us copy it directly into the target slice.
	// Otherwise Rust would rightfully complain about a mut borrow and a
	// non-mut borrow existing at the same time.

	let from_slice = unsafe { slice::from_raw_parts(from_stack_ptr.add(new_size), amt) };

	stacks[to].extend_from_slice(from_slice);
	stacks[from].truncate(new_size);
}

pub fn solve(input: &[u8]) -> Solution {
	let mut stacks_9000: [Vec<u8>; 9] = Default::default();

	let split_point = input
		.windows(2)
		.position(|window| window == b"\n\n")
		.expect("malformed input, expected \\n\\n");

	let (top, mut bottom) = input.split_at(split_point);

	bottom = &bottom[2..bottom.len() - 1];

	for line in top.split(|byte| *byte == b'\n').rev().skip(1) {
		line.chunks(4)
			.map(|chunk| chunk[1])
			.enumerate()
			.filter(|(_, id)| id.is_ascii_uppercase())
			.for_each(|(index, id)| stacks_9000[index].push(id))
	}

	let mut stacks_9001 = stacks_9000.clone();

	let mut instructions = bottom
		.split(|b| *b == b'\n' || *b == b' ')
		.enumerate()
		.filter(|(i, _)| i % 2 == 1)
		.map(|(_, b)| {
			let mut num = (b[0] - NUMBERS_START) as usize;
			for digit in &b[1..] {
				num = num * 10 + (digit - NUMBERS_START) as usize;
			}
			num
		});

	while let (Some(amt), Some(from), Some(to)) = (
		instructions.next(),
		instructions.next(),
		instructions.next(),
	) {
		let (from, to) = (from - 1, to - 1); // zero-based indices

		instruction_9000(&mut stacks_9000, amt, from, to);
		instruction_9001(&mut stacks_9001, amt, from, to);
	}

	Solution(get_peaks(&stacks_9000), get_peaks(&stacks_9001))
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution("CMZ".into(),"MCD".into()))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution("DHBJQJCCW".into(), "WJVRLSJJT".into()))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
