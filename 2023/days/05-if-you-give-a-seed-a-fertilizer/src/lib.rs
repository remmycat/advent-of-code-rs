use aoc_utils::{ascii_int::parse_uint_unchecked, iteration::expect_n};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug)]
struct Mapping {
	range: (usize, usize),
	offset: isize,
}

enum RangeMatch {
	Full {
		mapped: (usize, usize),
	},
	PartialOneSide {
		mapped: (usize, usize),
		side: (usize, usize),
	},
	PartialTwoSides {
		mapped: (usize, usize),
		side_1: (usize, usize),
		side_2: (usize, usize),
	},
}

impl Mapping {
	fn parse(line: &[u8]) -> Self {
		let [dest_start, source_start, len] = expect_n(
			line.split(|b| *b == b' ').map(parse_uint_unchecked),
			"bad input, expected 3 numbers in line",
		);

		Mapping {
			offset: dest_start as isize - source_start as isize,
			range: (source_start, source_start + len - 1),
		}
	}

	fn matches(&self, num: usize) -> bool {
		num >= self.range.0 && num <= self.range.1
	}

	fn map_range(&self, range: &(usize, usize)) -> Option<RangeMatch> {
		if range.1 < self.range.0 || range.0 > self.range.1 {
			None
		} else if range.0 >= self.range.0 && range.1 <= self.range.1 {
			Some(RangeMatch::Full {
				mapped: (self.apply(range.0), self.apply(range.1)),
			})
		} else if range.0 >= self.range.0 && range.1 > self.range.1 {
			// leftover to the right
			Some(RangeMatch::PartialOneSide {
				mapped: (self.apply(range.0), self.apply(self.range.1)),
				side: (self.range.1 + 1, range.1),
			})
		} else if range.0 < self.range.0 && range.1 <= self.range.1 {
			// leftover to the left
			Some(RangeMatch::PartialOneSide {
				side: (range.0, self.range.0 - 1),
				mapped: (self.apply(self.range.0), self.apply(range.1)),
			})
		} else {
			// leftover to both sides
			Some(RangeMatch::PartialTwoSides {
				side_1: (range.0, self.range.0 - 1),
				mapped: (self.apply(self.range.0), self.apply(self.range.1)),
				side_2: (self.range.1 + 1, range.1),
			})
		}
	}

	fn apply(&self, num: usize) -> usize {
		(num as isize + self.offset) as usize
	}
}

fn apply_mappings(mappings: &[Mapping], nums: &mut [usize], ranges: &mut Vec<(usize, usize)>) {
	for num in nums.iter_mut() {
		if let Some(mapping) = mappings.iter().find(|m| m.matches(*num)) {
			*num = mapping.apply(*num)
		}
	}

	let mut mapped_ranges: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());

	while let Some(range) = ranges.pop() {
		if let Some(range_match) = mappings
			.iter()
			.find_map(|mapping| mapping.map_range(&range))
		{
			match range_match {
				RangeMatch::Full { mapped } => mapped_ranges.push(mapped),
				RangeMatch::PartialOneSide { mapped, side } => {
					mapped_ranges.push(mapped);
					ranges.push(side);
				}
				RangeMatch::PartialTwoSides {
					mapped,
					side_1,
					side_2,
				} => {
					mapped_ranges.push(mapped);
					ranges.push(side_1);
					ranges.push(side_2);
				}
			}
		} else {
			// no mapping = stays the same
			mapped_ranges.push(range);
		}
	}

	*ranges = mapped_ranges;
}

pub fn solve(input: &[u8]) -> Solution {
	let mut nums: Vec<usize> = input
		.split(|b| *b == b'\n')
		.next()
		.expect("expected a newline")
		.split(|b| *b == b' ')
		.skip(1)
		.map(parse_uint_unchecked)
		.collect();

	let mut num_iter = nums.iter();
	let mut ranges: Vec<(usize, usize)> = vec![];
	while let (Some(a), Some(b)) = (num_iter.next(), num_iter.next()) {
		ranges.push((*a, *a + *b - 1));
	}

	let mut block_mappings: Vec<Mapping> = vec![];

	for line in input.split(|b| *b == b'\n') {
		if line.first().is_some_and(|b| b.is_ascii_digit()) {
			block_mappings.push(Mapping::parse(line));
		} else if !block_mappings.is_empty() {
			apply_mappings(&block_mappings, &mut nums, &mut ranges);
			block_mappings.clear();
		}
	}

	nums.sort_unstable();
	let lowest_location = nums[0];

	ranges.sort_unstable_by_key(|range| range.0);
	let lowest_range_location = ranges[0].0;

	Solution(lowest_location, lowest_range_location)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(35, 46),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(551761867,57451709),
	)]
	#[case::friend(
		include_bytes!("../inputs/friend.txt"),
		Solution(323142486,79874951),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
