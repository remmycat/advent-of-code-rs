struct Solution {
	gifted_houses: usize,
	robo_houses: usize,
}

struct House {
	coords: (i128, i128),
	presents: u128,
}

struct VisitsMap {
	santa: (i128, i128),
	robo: (i128, i128),
	houses: Vec<House>,
}

impl VisitsMap {
	fn new() -> Self {
		VisitsMap {
			santa: (0, 0),
			robo: (0, 0),
			houses: vec![House {
				coords: (0, 0),
				presents: 1,
			}],
		}
	}

	fn move_sleigh(&mut self, is_robo: bool, direction: char) {
		let (old_x, old_y) = if is_robo { self.robo } else { self.santa };
		let (new_x, new_y) = match direction {
			'^' => (old_x, old_y + 1),
			'v' => (old_x, old_y - 1),
			'>' => (old_x + 1, old_y),
			'<' => (old_x - 1, old_y),
			_ => panic!("bad input"),
		};

		if is_robo {
			self.robo = (new_x, new_y);
		} else {
			self.santa = (new_x, new_y);
		};

		let past_house = self
			.houses
			.iter_mut()
			.find(|House { coords: (x, y), .. }| *x == new_x && *y == new_y);

		if let Some(house) = past_house {
			house.presents += 1;
		} else {
			self.houses.push(House {
				coords: (new_x, new_y),
				presents: 1,
			});
		}
	}
}

fn solve(input: &str) -> Solution {
	let mut visits = VisitsMap::new();
	let mut visits_with_robo = VisitsMap::new();
	input.trim().chars().enumerate().for_each(|(i, c)| {
		visits.move_sleigh(false, c);
		visits_with_robo.move_sleigh(i % 2 == 1, c);
	});

	let gifted_houses = visits.houses.len();

	let robo_houses = visits_with_robo.houses.len();

	Solution {
		gifted_houses,
		robo_houses,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve(">").gifted_houses, 2);
		assert_eq!(solve("^>v<").gifted_houses, 4);
		assert_eq!(solve("^v^v^v^v^v").gifted_houses, 2);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/03.txt").unwrap();

		assert_eq!(solve(&input).gifted_houses, 2572);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve("^v").robo_houses, 3);
		assert_eq!(solve("^>v<").robo_houses, 3);
		assert_eq!(solve("^v^v^v^v^v").robo_houses, 11);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/03.txt").unwrap();

		assert_eq!(solve(&input).robo_houses, 2631);
	}
}
