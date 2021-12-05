struct Solution {
	floor: i128,
	first_time_basement: Option<usize>,
}

fn solve(input: &str) -> Solution {
	let mut first_time_basement = None;

	let floor = input.chars().enumerate().fold(0, |floor, (index, c)| {
		let floor = match c {
			'(' => floor + 1,
			')' => floor - 1,
			_ => panic!("bad input"),
		};
		if first_time_basement.is_none() && floor < 0 {
			first_time_basement = Some(index + 1);
		};

		floor
	});

	Solution {
		floor,
		first_time_basement,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("(())").floor, 0);
		assert_eq!(solve("()()").floor, 0);

		assert_eq!(solve("(((").floor, 3);
		assert_eq!(solve("(()(()(").floor, 3);
		assert_eq!(solve("))(((((").floor, 3);

		assert_eq!(solve("())").floor, -1);
		assert_eq!(solve("))(").floor, -1);

		assert_eq!(solve(")))").floor, -3);
		assert_eq!(solve(")())())").floor, -3);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2015/input_01.txt").unwrap();

		assert_eq!(solve(&input).floor, 232);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve(")").first_time_basement, Some(1));
		assert_eq!(solve("()())").first_time_basement, Some(5));
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2015/input_01.txt").unwrap();

		assert_eq!(solve(&input).first_time_basement, Some(1783));
	}
}
