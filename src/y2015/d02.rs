use std::str::FromStr;

use anyhow::Error;

struct Solution {
	square_feet: u128,
	ribbon_length: u128,
}

struct Measurements(u128, u128, u128);

impl FromStr for Measurements {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: [u128; 3] = s
			.split('x')
			.map(|m| m.parse::<u128>().unwrap())
			.collect::<Vec<_>>()
			.as_slice()
			.try_into()
			.unwrap();

		Ok(Measurements(parts[0], parts[1], parts[2]))
	}
}

fn solve(input: &str) -> Solution {
	let (square_feet, ribbon_length) = input
		.lines()
		.map(|line| line.parse::<Measurements>().unwrap())
		.fold(
			(0, 0),
			|(total_area, total_ribbon), Measurements(l, w, h)| {
				let side_areas = vec![l * w, l * h, w * h];
				let min_side = side_areas.iter().min().unwrap();
				let area = 2 * side_areas.iter().sum::<u128>() + min_side;

				let side_circumferences = vec![2 * (l + w), 2 * (l + h), 2 * (w + h)];
				let min_circumference = side_circumferences.iter().min().unwrap();
				let ribbon = min_circumference + l * w * h;

				(total_area + area, total_ribbon + ribbon)
			},
		);

	Solution {
		square_feet,
		ribbon_length,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("2x3x4").square_feet, 58);
		assert_eq!(solve("1x1x10").square_feet, 43);

		assert_eq!(solve("2x3x4\n1x1x10").square_feet, 58 + 43);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2015/input_02.txt").unwrap();

		assert_eq!(solve(&input).square_feet, 1598415);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve("2x3x4").ribbon_length, 34);
		assert_eq!(solve("1x1x10").ribbon_length, 14);

		assert_eq!(solve("2x3x4\n1x1x10").ribbon_length, 34 + 14);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2015/input_02.txt").unwrap();

		assert_eq!(solve(&input).ribbon_length, 3812909);
	}
}
