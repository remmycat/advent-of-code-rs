pub struct Solution {
	pub double_item_priority_sum: u64,
	pub badge_priority_sum: u64,
}

const LINE_SPLIT: u8 = b'\n';

const UPPERCASE_START: u8 = b'A' - 1;
const LOWERCASE_START: u8 = b'a' - 1;

const fn item_to_priority_flag(item: &u8) -> u64 {
	let priority = if *item > LOWERCASE_START {
		*item - LOWERCASE_START
	} else {
		*item - UPPERCASE_START + 26
	};
	1 << priority
}

const fn priority_flag_to_priority(flag: u64) -> u64 {
	flag.trailing_zeros() as u64
}

fn get_compartment_set(compartment: &[u8]) -> u64 {
	compartment
		.iter()
		.fold(0_u64, |set, item| (set | item_to_priority_flag(item)))
}

pub fn solve(input: &[u8]) -> Solution {
	let mut double_item_priority_sum: u64 = 0;
	let mut badge_priority_sum: u64 = 0;

	let mut badge_set: u64 = u64::MAX;

	for (elf_index, rucksack) in input
		.split(|b| *b == LINE_SPLIT)
		.filter(|l| !l.is_empty())
		.enumerate()
	{
		let (left, right) = rucksack.split_at(rucksack.len() / 2);

		let left_compartment_set = get_compartment_set(left);
		let right_compartment_set = get_compartment_set(right);

		double_item_priority_sum +=
			priority_flag_to_priority(left_compartment_set & right_compartment_set);

		// set all non-contained items to 0
		badge_set &= left_compartment_set | right_compartment_set;

		if elf_index % 3 == 2 {
			// At this point the badge set should only have a single bit set to 1
			badge_priority_sum += priority_flag_to_priority(badge_set);
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
