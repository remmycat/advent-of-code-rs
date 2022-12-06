use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(u64, u64);

fn has_duplicates(set: &mut HashSet<u8>, values: &[u8]) -> bool {
	set.insert(*(values.last().expect("expected at least 1 value")));

	let has_duplicates = set.len() != values.len();

	let mut iter = values.iter();

	let first = iter.next().expect("expected at least 1 value");

	// if none of the values are equal to the first one, it should get removed
	// from the set before the next iteration
	if iter.all(|v| v != first) {
		set.remove(first);
	};

	has_duplicates
}

fn count_until_n_different<const N: usize>(bytes: &[u8]) -> usize {
	let mut current_set = HashSet::new();

	// inititalise the set
	for byte in bytes.iter().take(N - 1) {
		current_set.insert(*byte);
	}

	bytes
		.windows(N)
		.position(|byte_window| !has_duplicates(&mut current_set, byte_window))
		.expect("bad input?")
}

pub fn solve(input: &[u8]) -> Solution {
	let until_sop_marker = count_until_n_different::<4>(input);
	// we can utilise that we already know that before this there cannot be 14 equal ones

	let until_som_marker = count_until_n_different::<14>(&input[until_sop_marker..]);

	let sop_bytes_read = until_sop_marker + 4;
	let som_bytes_read = until_som_marker + until_sop_marker + 14;

	Solution(sop_bytes_read as u64, som_bytes_read as u64)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example_5.txt"), Solution(5,23))]
	#[case(include_bytes!("../inputs/example_6.txt"), Solution(6,23))]
	#[case(include_bytes!("../inputs/example_7.txt"), Solution(7,19))]
	#[case(include_bytes!("../inputs/example_10.txt"), Solution(10,29))]
	#[case(include_bytes!("../inputs/example_11.txt"), Solution(11,26))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(1142,2803))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
