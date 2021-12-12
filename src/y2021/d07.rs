pub struct Solution {
	lowest_cost_misunderstood: i32,
	lowest_cost_understood: i32,
}

fn fuel_sum(num: i32) -> i32 {
	// Naive approach:
	// (0..=of).into_iter().sum()
	// but this gives the same result and is much faster. thanks maths 101
	(num as f64 * (num as f64 + 1.0) / 2.0) as i32
}

pub fn solve(input: &str) -> Solution {
	let nums: Vec<i32> = input
		.split(',')
		.map(|n| n.parse::<i32>().unwrap())
		.collect();

	let max = nums.iter().max().unwrap();

	// alt analytic solution: calculate fuel for position at median
	// found by myself
	let lowest_cost_misunderstood = (0..*max)
		.into_iter()
		.map(|pos| nums.iter().map(|num| (*num - pos).abs()).sum())
		.min()
		.unwrap();

	// alt analytic solution: (num_crabs - 2*(num_values_smaller_than_mean))/(2*num_crabs)
	// by https://www.reddit.com/r/adventofcode/comments/rav728/comment/hnkvnzr/
	let lowest_cost_understood = (0..*max)
		.into_iter()
		.map(|pos| nums.iter().map(|num| fuel_sum((*num - pos).abs())).sum())
		.min()
		.unwrap();

	Solution {
		lowest_cost_misunderstood,
		lowest_cost_understood,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("16,1,2,0,4,2,7,1,2,14").lowest_cost_misunderstood, 37);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_07.txt").unwrap();

		assert_eq!(solve(input.trim()).lowest_cost_misunderstood, 331067);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve("16,1,2,0,4,2,7,1,2,14").lowest_cost_understood, 168);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_07.txt").unwrap();

		assert_eq!(solve(input.trim()).lowest_cost_understood, 92881128);
	}
}
