#[derive(Debug, PartialEq, Eq)]
pub struct Solution(u64, u64);

#[derive(Debug, PartialEq, Eq)]
struct Point(usize, usize);

fn tree_outlook<'a>(
	tree: &'a u8,
	trees_in_sight: impl Iterator<Item = &'a u8>,
	scenic_score: &mut u64,
	is_visible: &mut bool,
) {
	let mut scenic_distance = 0_u64;
	let mut scenic_found = false;
	let mut all_smaller = true;

	for other_tree in trees_in_sight {
		let smaller = other_tree < tree;
		if !scenic_found {
			scenic_distance += 1;
			if !smaller {
				scenic_found = true;
			}
		}

		if !smaller {
			all_smaller = false;
		}

		if scenic_found && (*is_visible || !all_smaller) {
			// nothing more to find here
			break;
		}
	}

	*scenic_score *= scenic_distance;

	if all_smaller {
		*is_visible = true;
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let mut visible: u64 = 0;

	let width = input
		.iter()
		.position(|b| *b == b'\n')
		.expect("bad input, expected at leat 2 rows");

	let trees: Vec<_> = input.iter().filter(|b| **b != b'\n').collect();

	let mut best_scenic_score = 0_u64;

	let height = trees.len() / width;

	// borders
	visible += (width * 2 + height * 2 - 4) as u64;

	for y in 1..(height - 1) {
		let row_index = y * width;

		for x in 1..(width - 1) {
			let tree_index = row_index + x;
			let tree = trees[tree_index];

			let mut scenic_score = 1_u64;
			let mut is_visible = false;

			let to_left = (row_index..tree_index).rev().map(|pos| trees[pos]);
			let to_right = ((tree_index + 1)..(row_index + width)).map(|pos| trees[pos]);
			let to_top = (0..y).rev().map(|y| trees[y * width + x]);
			let to_bot = ((y + 1)..height).map(|y| trees[y * width + x]);

			tree_outlook(tree, to_left, &mut scenic_score, &mut is_visible);
			tree_outlook(tree, to_right, &mut scenic_score, &mut is_visible);
			tree_outlook(tree, to_top, &mut scenic_score, &mut is_visible);
			tree_outlook(tree, to_bot, &mut scenic_score, &mut is_visible);

			if scenic_score > best_scenic_score {
				best_scenic_score = scenic_score;
			}

			if is_visible {
				visible += 1;
			}
		}
	}

	Solution(visible, best_scenic_score)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(21,8))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(1705,371200))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
