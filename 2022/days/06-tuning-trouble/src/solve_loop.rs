use crate::Solution;

fn count_until_n_different_loop<const N: usize>(bytes: &[u8]) -> usize {
	bytes
		.windows(N)
		.position(|window| {
			for (index, a) in window.iter().enumerate() {
				for b in &window[(index + 1)..] {
					if a == b {
						return false;
					}
				}
			}

			true
		})
		.expect("bad input?")
}

pub fn solve_loop(input: &[u8]) -> Solution {
	let until_sop_marker = count_until_n_different_loop::<4>(input);
	// we can utilise that we already know that before this there cannot be 14 equal ones

	let until_som_marker = count_until_n_different_loop::<14>(&input[until_sop_marker..]);

	let sop_bytes_read = until_sop_marker + 4;
	let som_bytes_read = until_som_marker + until_sop_marker + 14;

	Solution(sop_bytes_read as u64, som_bytes_read as u64)
}
