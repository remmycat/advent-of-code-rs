struct Solution {
	depth_increases: u128,
	average_depth_increases: u128,
}

fn solve(input: &str) -> Solution {
	let measurements: Vec<_> = input.lines().map(|l| l.parse::<i128>().unwrap()).collect();

	let mut solution = Solution {
		depth_increases: 0,
		average_depth_increases: 0,
	};

	for (i, m) in measurements.iter().enumerate() {
		let l1 = if i > 0 { measurements.get(i - 1) } else { None };
		let l3 = if i > 2 { measurements.get(i - 3) } else { None };

		if let Some(last) = l1 {
			if m > last {
				solution.depth_increases += 1;
			}
		}

		// checking `m + l1 + l2 > l1 + l2 + l3` can be simplified to `m > l3`
		if let Some(last3) = l3 {
			if m > last3 {
				solution.average_depth_increases += 1;
			}
		}
	}

	solution
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

		assert_eq!(solve(example).depth_increases, 7)
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_01.txt").unwrap();

		assert_eq!(solve(&input).depth_increases, 1342);
	}

	#[test]
	fn part_2_example_cases() {
		let example = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

		assert_eq!(solve(example).average_depth_increases, 5)
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_01.txt").unwrap();

		assert_eq!(solve(&input).average_depth_increases, 1378);
	}
}
