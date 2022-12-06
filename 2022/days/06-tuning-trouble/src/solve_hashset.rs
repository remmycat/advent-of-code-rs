use crate::Solution;
use std::collections::HashSet;

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

fn count_until_n_different_hashset<const N: usize>(bytes: &[u8]) -> usize {
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

pub fn solve_hashset(input: &[u8]) -> Solution {
	let until_sop_marker = count_until_n_different_hashset::<4>(input);
	// we can utilise that we already know that before this there cannot be 14 equal ones

	let until_som_marker = count_until_n_different_hashset::<14>(&input[until_sop_marker..]);

	let sop_bytes_read = until_sop_marker + 4;
	let som_bytes_read = until_som_marker + until_sop_marker + 14;

	Solution(sop_bytes_read as u64, som_bytes_read as u64)
}
