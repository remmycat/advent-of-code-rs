use itertools::Itertools;

pub struct Solution {
	extreme_element_diff_10: usize,
	extreme_element_diff_40: usize,
}

// convert uppercase ascii char to 0..25
fn get_char_code(c: char) -> u8 {
	// debug_assert!(c.is_ascii_alphabetic() && c.is_ascii_uppercase());
	c as u8 - 65
}

fn polymerize(
	atom_pair_count: &mut [[usize; 26]; 26],
	insertion_map: &[[u8; 26]; 26],
	last_char: usize,
	steps: usize,
) -> Vec<usize> {
	for _step in 1..=steps {
		let mut step_count = [[0_usize; 26]; 26];
		for a in 0..26 {
			for b in 0..26 {
				let count = atom_pair_count[a][b];
				if count != 0 {
					let mid = insertion_map[a][b] as usize;
					step_count[a][mid] += count;
					step_count[mid][b] += count;
				}
			}
		}
		for a in 0..26 {
			for b in 0..26 {
				atom_pair_count[a][b] = step_count[a][b];
			}
		}
	}

	atom_pair_count
		.iter()
		.enumerate()
		.map(|(a, b_map)| -> usize {
			let a_count = b_map.iter().sum();
			if a == last_char {
				// correction for only counting the left sides of the result
				a_count + 1
			} else {
				a_count
			}
		})
		.filter(|count| *count > 0)
		.collect()
}

pub fn solve(input: &str) -> Solution {
	let mut input_parts = input.split("\n\n");
	let atoms = input_parts.next().unwrap();
	let insertions = input_parts.next().unwrap();

	let mut insertion_map = [[0_u8; 26]; 26];
	let mut atom_pair_count = [[0_usize; 26]; 26];

	for line in insertions.lines() {
		let mut chars = line.chars();
		// XY -> Z
		// 0123456

		let a = get_char_code(chars.next().unwrap());
		let b = get_char_code(chars.next().unwrap());
		let middle = get_char_code(chars.nth(4).unwrap());

		insertion_map[a as usize][b as usize] = middle;
	}

	let inital_atom_pairs: Vec<(_, _)> = atoms.chars().map(get_char_code).tuple_windows().collect();
	let last_char = inital_atom_pairs.last().unwrap().1 as usize;

	for (a, b) in inital_atom_pairs.clone() {
		atom_pair_count[a as usize][b as usize] += 1;
	}

	let counts_10 = polymerize(&mut atom_pair_count, &insertion_map, last_char, 10);
	let extreme_element_diff_10 = counts_10.iter().max().unwrap() - counts_10.iter().min().unwrap();
	// We already counted 10 steps into "atom_pair_count", no need to reset & recount.
	let counts_40 = polymerize(&mut atom_pair_count, &insertion_map, last_char, 40 - 10);
	let extreme_element_diff_40 = counts_40.iter().max().unwrap() - counts_40.iter().min().unwrap();

	Solution {
		extreme_element_diff_10,
		extreme_element_diff_40,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let input = fs::read_to_string("assets/2021/input_14_sample.txt").unwrap();

		assert_eq!(solve(&input).extreme_element_diff_10, 1588);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_14.txt").unwrap();

		assert_eq!(solve(&input).extreme_element_diff_10, 3697);
	}

	#[test]
	fn part_2_example_cases() {
		let input = fs::read_to_string("assets/2021/input_14_sample.txt").unwrap();

		assert_eq!(solve(&input).extreme_element_diff_40, 2188189693529);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_14.txt").unwrap();

		assert_eq!(solve(&input).extreme_element_diff_40, 4371307836157);
	}
}
