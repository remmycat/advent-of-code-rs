use std::str::FromStr;

pub struct Solution {
	y_max: i32,
	possible_velocities: usize,
}

fn parse_range(range_s: &str) -> (i32, i32) {
	let (min_s, max_s) = range_s.split_once("..").unwrap();
	let min = i32::from_str(min_s).unwrap();
	let max = i32::from_str(max_s).unwrap();

	(min, max)
}

// This code is horrible because most of the math happened in my head. 🤷

pub fn solve(input: &str) -> Solution {
	let (_, ranges) = input.split_once("target area: x=").unwrap();
	let (x_range_s, y_range_s) = ranges.split_once(", y=").unwrap();
	let (x_start, x_end) = parse_range(x_range_s);
	let (y_start, y_end) = parse_range(y_range_s);
	let y_range = y_start..=y_end;
	let x_range = x_start..=x_end;

	assert!(y_start < 0);
	assert!(y_end < 0);
	assert!(x_start > 0);
	assert!(x_end > 0);

	// if the y_range is negative we need the biggest yv so that -1 * yv+1 is still inside the range
	// so the highest yv to achieve this is yv = -1 * y_range.start() -1

	let y_vel_max = -(y_start) - 1;
	// y_max is gonna be the point where y_vel_max == 0, which is exactly after y_vel_max steps.
	// since y(at ymax) == y_vel_max + y_vel_max - 1 + ... + 3 + 2 + 1 + 0, it's the same as triangular numbers.
	let y_max = (y_vel_max * (y_vel_max + 1)) / 2;

	let y_vel_min = y_start;

	let x_vel_max = x_end;
	// not true, but whatever.
	let x_vel_min = 0;

	let possible_x_vels: Vec<_> = (x_vel_min..=x_vel_max)
		.filter_map(|x_vel| {
			let mut sim_x = 0;
			let mut sim_vel = x_vel;
			let mut step = 1;

			let mut step_start = None;
			let mut step_end = None;

			loop {
				sim_x += sim_vel;
				if x_range.contains(&sim_x) {
					if step_start.is_none() {
						step_start.replace(step);
					}
					step_end.replace(step);
				}
				if sim_x > x_end {
					return step_start.map(|step_start| {
						let step_end = step_end.unwrap();
						(x_vel, step_start, step_end)
					});
				} else if sim_vel == 0 {
					return step_start.map(|step_start| (x_vel, step_start, i32::MAX));
				}
				sim_vel -= 1;
				step += 1;
			}
		})
		.collect();

	let possible_velocities = (0..=y_vel_max)
		.filter_map(|y_vel| {
			let mut sim_y = 0;
			let mut sim_vel = y_vel;
			let mut step = 1;

			let mut step_start = None;
			let mut step_end = None;
			loop {
				sim_vel += 1;
				sim_y -= sim_vel;
				if y_range.contains(&sim_y) {
					if step_start.is_none() {
						step_start.replace(step);
					}
					step_end.replace(step);
				}
				if sim_y < y_vel_min {
					return step_start.map(|step_start| {
						let step_end = step_end.unwrap();
						let a_step_offset = 2 * y_vel + 1;
						let vel_a = (y_vel, a_step_offset + step_start, a_step_offset + step_end);
						let vel_b = (-(y_vel + 1), step_start, step_end);
						[vel_a, vel_b].into_iter()
					});
				}
				step += 1;
			}
		})
		.flatten()
		.map(|(_, y_steps_start, y_steps_end)| -> usize {
			possible_x_vels
				.iter()
				.filter(|(_, x_steps_start, x_steps_end)| {
					*x_steps_start <= y_steps_end && y_steps_start <= *x_steps_end
				})
				.count()
		})
		.sum();

	Solution {
		y_max,
		possible_velocities,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("target area: x=20..30, y=-10..-5").y_max, 45);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/17.txt").unwrap();

		assert_eq!(solve(&input).y_max, 10296);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(
			solve("target area: x=20..30, y=-10..-5").possible_velocities,
			112
		);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/17.txt").unwrap();

		assert_eq!(solve(&input).possible_velocities, 2371);
	}
}
