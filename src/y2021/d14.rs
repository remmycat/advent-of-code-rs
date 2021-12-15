use std::iter::Peekable;

use itertools::Itertools;

pub struct Solution {
	extreme_element_diff_10: usize,
	extreme_element_diff_40: usize,
}

pub trait IdentifyLast: Iterator + Sized {
	fn identify_last(self) -> Iter<Self>;
}

impl<I> IdentifyLast for I
where
	I: Iterator,
{
	fn identify_last(self) -> Iter<Self> {
		Iter(self.peekable())
	}
}

pub struct Iter<I>(Peekable<I>)
where
	I: Iterator;

impl<I> Iterator for Iter<I>
where
	I: Iterator,
{
	type Item = (bool, I::Item);

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|e| (self.0.peek().is_none(), e))
	}
}

// convert uppercase ascii char to 0..25
fn get_char_code(c: char) -> u8 {
	// debug_assert!(c.is_ascii_alphabetic() && c.is_ascii_uppercase());
	c as u8 - 65
}

#[inline]
fn polymerization(
	insertion_map: &[[u8; 26]; 26],
	count_memo: &mut [[Option<[usize; 26]>; 26]; 26],
	element_collection: &mut [usize; 26],
	goal_depth: u8,
	depth: u8,
	a: u8,
	b: u8,
) {
	if depth == goal_depth {
		// only count left side, right would otherwise be double-counted.
		element_collection[a as usize] += 1;
	} else {
		let middle = insertion_map[a as usize][b as usize];

		// let's memo at the half point
		if depth == goal_depth / 2 {
			if count_memo[a as usize][b as usize].is_none() {
				// Okay, gonna have to memoize our result for the next person
				let mut memo_collection = [0; 26];
				polymerization(
					insertion_map,
					count_memo,
					&mut memo_collection,
					goal_depth,
					depth + 1,
					a,
					middle,
				);
				polymerization(
					insertion_map,
					count_memo,
					&mut memo_collection,
					goal_depth,
					depth + 1,
					middle,
					b,
				);
				for (index, count) in memo_collection.iter().enumerate() {
					element_collection[index] += count;
				}
				count_memo[a as usize][b as usize] = Some(memo_collection);
			} else {
				let memoized_counts = count_memo[a as usize][b as usize].unwrap();

				for (index, count) in memoized_counts.iter().enumerate() {
					element_collection[index] += count;
				}
			}
		} else {
			polymerization(
				insertion_map,
				count_memo,
				element_collection,
				goal_depth,
				depth + 1,
				a,
				middle,
			);
			polymerization(
				insertion_map,
				count_memo,
				element_collection,
				goal_depth,
				depth + 1,
				middle,
				b,
			);
		}
	}
}

pub fn solve(input: &str) -> Solution {
	let mut insertion_map = [[0_u8; 26]; 26];

	let mut count_memo = [[None; 26]; 26];

	let mut parts = input.split("\n\n");

	let atoms = parts.next().unwrap();
	let insertions = parts.next().unwrap();

	for line in insertions.lines() {
		let mut chars = line.chars();
		// XY -> Z
		// 0123456

		let a = get_char_code(chars.next().unwrap());
		let b = get_char_code(chars.next().unwrap());
		let middle = get_char_code(chars.nth(4).unwrap());

		insertion_map[a as usize][b as usize] = middle;
	}

	let inital_atom_tuples: Vec<(u8, u8)> =
		atoms.chars().map(get_char_code).tuple_windows().collect();
	let last_char = inital_atom_tuples.last().unwrap().1;

	let mut element_collection_10 = [0_usize; 26];

	// correction for only counting the left sides of the result
	element_collection_10[last_char as usize] = 1;

	for (a, b) in inital_atom_tuples.clone() {
		polymerization(
			&insertion_map,
			&mut count_memo,
			&mut element_collection_10,
			10,
			0,
			a,
			b,
		);
	}

	let extreme_element_diff_10 = element_collection_10.iter().max().unwrap()
		- element_collection_10
			.into_iter()
			.filter(|count| *count != 0)
			.min()
			.unwrap();

	count_memo = [[None; 26]; 26];

	let mut element_collection_40 = [0_usize; 26];

	// correction for only counting the left sides of the result
	element_collection_40[last_char as usize] = 1;

	for (a, b) in inital_atom_tuples {
		polymerization(
			&insertion_map,
			&mut count_memo,
			&mut element_collection_40,
			40,
			0,
			a,
			b,
		);
	}

	let extreme_element_diff_40 = element_collection_40.iter().max().unwrap()
		- element_collection_40
			.into_iter()
			.filter(|count| *count != 0)
			.min()
			.unwrap();

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

		assert_eq!(solve(&input).extreme_element_diff_40, 0);
	}
}
