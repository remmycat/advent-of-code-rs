pub struct Solution {
	pub double_item_priority_sum: u64,
	pub badge_priority_sum: u64,
}

const LINE_SPLIT: u8 = b'\n';

const UPPERCASE_START: u8 = b'A' - 1;
const LOWERCASE_START: u8 = b'a' - 1;

const fn item_to_priority(item: &u8) -> u8 {
	if *item > LOWERCASE_START {
		*item - LOWERCASE_START
	} else {
		*item - UPPERCASE_START + 26
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let mut double_item_priority_sum: u64 = 0;
	let mut badge_priority_sum: u64 = 0;
	// These "sets" use bitflags for each possible item priority.
	let mut compartement_item_set: u64;
	let mut full_item_set: u64;
	let mut badge_set: u64 = u64::MAX;

	for (elf_index, rucksack) in input
		.split(|b| *b == LINE_SPLIT)
		.filter(|l| !l.is_empty())
		.enumerate()
	{
		let size = rucksack.len();
		let compartement_size = size / 2;

		compartement_item_set = 0;
		full_item_set = 0;

		for (item_index, priority) in rucksack.iter().map(item_to_priority).enumerate() {
			let priority_flag: u64 = 1 << priority;

			full_item_set |= priority_flag;

			if item_index < compartement_size {
				compartement_item_set |= priority_flag;
				continue;
			}

			if (compartement_item_set & priority_flag) == priority_flag {
				// found duplicate
				double_item_priority_sum += priority as u64;
				// We cannot break out of the loop because we need to build up
				// full_item_set.
				// We could set a flag here to stop processing this part of
				// the loop, but not doing that turns out to be faster, I guess
				// due to branch prediction.
			}
		}

		// set all non-contained item priorities to 0
		badge_set &= full_item_set;

		if elf_index % 3 == 2 {
			// At this point the badge set should only have a single bit set to 1
			// Since priority X means a 1 shifted X positions to the left, the inverse
			// operation (to get the priority back) is counting the trailing zeros.
			badge_priority_sum += badge_set.trailing_zeros() as u64;
			badge_set = u64::MAX;
		}
	}

	Solution {
		double_item_priority_sum,
		badge_priority_sum,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::include_bytes;

	const EXAMPLE: &[u8] = include_bytes!("../inputs/example.txt");
	const PERSONAL: &[u8] = include_bytes!("../inputs/personal.txt");

	#[test]
	fn part_1_example() {
		assert_eq!(solve(EXAMPLE).double_item_priority_sum, 157);
	}

	#[test]
	fn part_1_personal() {
		assert_eq!(solve(PERSONAL).double_item_priority_sum, 7446);
	}

	#[test]
	fn part_2_example() {
		assert_eq!(solve(EXAMPLE).badge_priority_sum, 70);
	}

	#[test]
	fn part_2_personal() {
		assert_eq!(solve(PERSONAL).badge_priority_sum, 2646);
	}
}
