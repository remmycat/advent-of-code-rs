use std::str::FromStr;

pub struct Solution {
	pub full_overlap_pairs_count: u64,
	pub overlap_pairs_count: u64,
}

struct SectionRange(u8, u8);

impl FromStr for SectionRange {
	type Err = &'static str;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let (min, max) = value.split_once('-').ok_or("malformed range")?;
		let (min, max) = (
			min.parse().map_err(|_| "malformed min")?,
			max.parse().map_err(|_| "malformed max")?,
		);
		Ok(SectionRange(min, max))
	}
}

impl SectionRange {
	fn overlaps_start(&self, other: &SectionRange) -> bool {
		self.0 >= other.0 && self.0 <= other.1
	}

	fn contains(&self, other: &SectionRange) -> bool {
		self.0 <= other.0 && self.1 >= other.1
	}
}

struct AssignmentPair(SectionRange, SectionRange);

impl FromStr for AssignmentPair {
	type Err = &'static str;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let (left, right) = value.split_once(',').ok_or("malformed pair")?;
		let (left, right) = (left.parse()?, right.parse()?);
		Ok(AssignmentPair(left, right))
	}
}

impl AssignmentPair {
	fn has_overlap(&self) -> bool {
		let AssignmentPair(left, right) = self;
		left.overlaps_start(right) || right.overlaps_start(left)
	}
	fn has_full_overlap(&self) -> bool {
		let AssignmentPair(left, right) = self;
		left.contains(right) || right.contains(left)
	}
}

pub fn solve(input: &str) -> Solution {
	let mut pairs: Vec<_> = input
		.trim()
		.lines()
		.map(AssignmentPair::from_str)
		.map(|pair| pair.expect("malformed input line"))
		.collect();

	pairs.retain(AssignmentPair::has_overlap);

	let overlap_pairs_count = pairs.len() as u64;

	let full_overlap_pairs_count = pairs
		.into_iter()
		.filter(AssignmentPair::has_full_overlap)
		.count() as u64;

	Solution {
		full_overlap_pairs_count,
		overlap_pairs_count,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::include_str;

	const EXAMPLE: &str = include_str!("../inputs/example.txt");
	const PERSONAL: &str = include_str!("../inputs/personal.txt");

	#[test]
	fn part_1_example() {
		assert_eq!(solve(EXAMPLE).full_overlap_pairs_count, 2);
	}

	#[test]
	fn part_1_personal() {
		assert_eq!(solve(PERSONAL).full_overlap_pairs_count, 433);
	}

	#[test]
	fn part_2_example() {
		assert_eq!(solve(EXAMPLE).overlap_pairs_count, 4);
	}

	#[test]
	fn part_2_personal() {
		assert_eq!(solve(PERSONAL).overlap_pairs_count, 0);
	}
}
