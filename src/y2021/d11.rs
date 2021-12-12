pub struct Solution {
	total_flashes_100: usize,
	first_mega_flash: Option<usize>,
}

fn count_octopile(octopi: &[u8], x_size: usize, xy: usize) -> u8 {
	let cur_x = xy % x_size;
	// has top
	let t = xy >= x_size;
	// has left
	let l = cur_x > 0;
	// has right
	let r = cur_x < x_size - 1;
	// has bottom
	let b = xy < octopi.len() - x_size;

	[
		(t && l && octopi[xy - x_size - 1] > 9) as u8,
		(t && octopi[xy - x_size] > 9) as u8,
		(t && r && octopi[xy - x_size + 1] > 9) as u8,
		(l && octopi[xy - 1] > 9) as u8,
		(r && octopi[xy + 1] > 9) as u8,
		(b && l && octopi[xy + x_size - 1] > 9) as u8,
		(b && octopi[xy + x_size] > 9) as u8,
		(b && r && octopi[xy + x_size + 1] > 9) as u8,
	]
	.into_iter()
	.sum()
}

pub fn solve(input: &str) -> Solution {
	let rows: Vec<_> = input.lines().collect();
	let x_size = rows.get(0).unwrap().len();

	let mut octopi: Vec<_> = rows
		.into_iter()
		.flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
		.collect();
	let mut total_flashes_100 = 0;
	let mut flash_mob = false;
	let mut first_mega_flash = None;

	for step in 1.. {
		octopi.iter_mut().for_each(|oct| {
			if *oct == 9 {
				flash_mob = true;
			}
			*oct += 1
		});

		while flash_mob {
			flash_mob = false;
			let nextopi: Vec<_> = octopi
				.iter()
				.enumerate()
				.map(|(xy, oct)| {
					let oct = *oct;

					if oct == 0 {
						0
					} else if oct > 9 {
						if step <= 100 {
							total_flashes_100 += 1;
						}
						0
					} else {
						let flashes = count_octopile(&octopi, x_size, xy);
						let next = oct + flashes;
						if next > 9 {
							flash_mob = true;
						}
						next
					}
				})
				.collect();

			octopi = nextopi;
		}

		if first_mega_flash.is_none() && octopi.iter().all(|oct| *oct == 0) {
			first_mega_flash = Some(step);
			break;
		}
	}

	Solution {
		total_flashes_100,
		first_mega_flash,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let big = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

		assert_eq!(solve(big.trim()).total_flashes_100, 1656);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_11.txt").unwrap();

		assert_eq!(solve(input.trim()).total_flashes_100, 1655);
	}

	#[test]
	fn part_2_example_cases() {
		let big = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

		assert_eq!(solve(big.trim()).first_mega_flash, Some(195));
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_11.txt").unwrap();

		assert_eq!(solve(input.trim()).first_mega_flash, Some(337));
	}
}
