use hashbrown::HashSet;

pub struct Solution {
	risk_level_sum: u32,
	top_3_multiplied: u128,
}

const MAX_LEVEL: u8 = 9;

struct Surroundings {
	top_pos: Option<usize>,
	bottom_pos: Option<usize>,
	left_pos: Option<usize>,
	right_pos: Option<usize>,
	top: u8,
	bottom: u8,
	left: u8,
	right: u8,
}

fn get_surroundings(points: &[u8], x_size: usize, xy: usize) -> Surroundings {
	let x = xy % x_size;
	let top_pos = if xy >= x_size {
		Some(xy - x_size)
	} else {
		None
	};
	let bottom_pos = if xy < points.len() - x_size {
		Some(xy + x_size)
	} else {
		None
	};
	let left_pos = if x > 0 { Some(xy - 1) } else { None };
	let right_pos = if x < x_size - 1 { Some(xy + 1) } else { None };

	Surroundings {
		top_pos,
		bottom_pos,
		left_pos,
		right_pos,
		top: top_pos.map(|pos| points[pos]).unwrap_or(MAX_LEVEL),
		bottom: bottom_pos.map(|pos| points[pos]).unwrap_or(MAX_LEVEL),
		left: left_pos.map(|pos| points[pos]).unwrap_or(MAX_LEVEL),
		right: right_pos.map(|pos| points[pos]).unwrap_or(MAX_LEVEL),
	}
}

pub fn solve(input: &str) -> Solution {
	let rows: Vec<_> = input.lines().collect();
	let x_size = rows.get(0).unwrap().len();

	let mut points: Vec<_> = rows
		.into_iter()
		.flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
		.collect();

	let risk_level_sum = points
		.iter()
		.enumerate()
		.filter_map(|(xy, point)| {
			let Surroundings {
				top,
				right,
				left,
				bottom,
				..
			} = get_surroundings(&points, x_size, xy);

			if top > *point && right > *point && bottom > *point && left > *point {
				Some((point + 1) as u32)
			} else {
				None
			}
		})
		.sum();

	let mut basin_sizes: Vec<u128> = Vec::new();
	let mut current_basin: HashSet<usize> = HashSet::new();
	let mut current_basin_next: Vec<usize> = Vec::new();

	// Strategy:
	// - Find next basin by getting first non-9 and putting it in a stack
	// - Work through the stack by
	//   - counting the point
	//   - mutating it to a 9 in the source
	//   - adding the surrounding non-9s to the stac
	while let Some(basin_start_index) = points.iter().position(|point| *point != MAX_LEVEL) {
		current_basin_next.push(basin_start_index);
		while let Some(xy) = current_basin_next.pop() {
			if !current_basin.contains(&xy) {
				let s = get_surroundings(&points, x_size, xy);

				if s.top != MAX_LEVEL {
					current_basin_next.push(s.top_pos.unwrap());
				}
				if s.bottom != MAX_LEVEL {
					current_basin_next.push(s.bottom_pos.unwrap());
				}
				if s.left != MAX_LEVEL {
					current_basin_next.push(s.left_pos.unwrap());
				}
				if s.right != MAX_LEVEL {
					current_basin_next.push(s.right_pos.unwrap());
				}

				current_basin.insert(xy);
			}
			// We're done with this point, let's overwrite it with a 9.
			points[xy] = MAX_LEVEL;
		}

		basin_sizes.push(current_basin.len() as u128);
		current_basin.clear();
	}

	// sort desc
	basin_sizes.sort_unstable_by(|a, b| b.cmp(a));

	let top_3_multiplied = basin_sizes.iter().take(3).product();

	Solution {
		risk_level_sum,
		top_3_multiplied,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;
		assert_eq!(solve(example.trim()).risk_level_sum, 15);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/09.txt").unwrap();

		assert_eq!(solve(input.trim()).risk_level_sum, 603);
	}

	#[test]
	fn part_2_example_cases() {
		let example = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;
		assert_eq!(solve(example.trim()).top_3_multiplied, 1134);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/09.txt").unwrap();

		assert_eq!(solve(input.trim()).top_3_multiplied, 786780);
	}
}
