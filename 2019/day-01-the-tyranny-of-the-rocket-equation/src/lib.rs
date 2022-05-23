pub struct Solution {
	pub core_fuel_requirements: u64,
	pub full_fuel_requirements: u64,
}

fn calculate_fuel(module_mass: u64) -> Option<u64> {
	(module_mass / 3).checked_sub(2)
}

fn calculate_fuel_fuel(last_fuel_mass: u64) -> u64 {
	match calculate_fuel(last_fuel_mass) {
		Some(fuel_mass) => last_fuel_mass + calculate_fuel_fuel(fuel_mass),
		None => last_fuel_mass,
	}
}

pub fn solve(input: &str) -> Solution {
	let fuel_mass_list: Vec<u64> = input
		.lines()
		.map(|line| {
			line.parse::<u64>()
				.expect("Could not parse input line as u128")
		})
		.map(|module_mass| {
			calculate_fuel(module_mass)
				.expect("Unexpected negative fuel consumption in main inputs")
		})
		.collect();

	let core_fuel_requirements = fuel_mass_list.iter().sum();

	let full_fuel_requirements = fuel_mass_list.into_iter().map(calculate_fuel_fuel).sum();

	Solution {
		core_fuel_requirements,
		full_fuel_requirements,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("12").core_fuel_requirements, 2);
		assert_eq!(solve("14").core_fuel_requirements, 2);
		assert_eq!(solve("1969").core_fuel_requirements, 654);
		assert_eq!(solve("100756").core_fuel_requirements, 33583);

		let combined_examples =
			fs::read_to_string("inputs/examples.txt").expect("Error reading example input file");
		let combined_result = 2 + 2 + 654 + 33583;

		assert_eq!(
			solve(&combined_examples).core_fuel_requirements,
			combined_result
		);
	}

	#[test]
	fn part_1_solution() {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).core_fuel_requirements, 3210097);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve("12").full_fuel_requirements, 2);
		assert_eq!(solve("14").full_fuel_requirements, 2);
		assert_eq!(solve("1969").full_fuel_requirements, 966);
		assert_eq!(solve("100756").full_fuel_requirements, 50346);

		let combined_examples =
			fs::read_to_string("inputs/examples.txt").expect("Error reading example input file");
		let combined_result = 2 + 2 + 966 + 50346;

		assert_eq!(
			solve(&combined_examples).full_fuel_requirements,
			combined_result
		);
	}

	#[test]
	fn part_2_solution() {
		let input =
			fs::read_to_string("inputs/personal.txt").expect("Error reading personal input file");

		assert_eq!(solve(&input).full_fuel_requirements, 4812287);
	}
}
