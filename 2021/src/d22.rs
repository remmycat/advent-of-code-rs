use std::ops::RangeInclusive;

pub struct Solution {
	cubes_on_in_center: u64,
	cubes_on: u64,
}

#[derive(Clone)]
struct RebootStep {
	on: bool,
	x_range: RangeInclusive<i32>,
	y_range: RangeInclusive<i32>,
	z_range: RangeInclusive<i32>,
}

enum Split {
	X { y: i32, z: i32 },
	Y { x: i32, z: i32 },
	Z { x: i32, y: i32 },
}

impl RebootStep {
	fn intersects_exclusive(&self, other: &RebootStep) -> bool {
		self.x_range.start() < other.x_range.end()
			&& other.x_range.start() < self.x_range.end()
			&& self.y_range.start() < other.y_range.end()
			&& other.y_range.start() < self.y_range.end()
			&& self.z_range.start() < other.z_range.end()
			&& other.z_range.start() < self.z_range.end()
	}

	/// splinter self into lots of cuboids, by looking at intersections with other step
	fn splinter(self, other: &RebootStep) -> Vec<RebootStep> {
		let splits = other.get_split_lines();

		let mut step_splinters = vec![self];

		for split in splits {
			step_splinters = step_splinters
				.into_iter()
				.flat_map(|step| match split {
					Split::X { y, z } => {
						if !step.y_range.contains(&y) || !step.z_range.contains(&z) {
							vec![step]
						} else {
							vec![
								RebootStep {
									on: step.on,
									x_range: step.x_range.clone(),
									y_range: *step.y_range.start()..=y,
									z_range: *step.z_range.start()..=z,
								},
								RebootStep {
									on: step.on,
									x_range: step.x_range.clone(),
									y_range: y..=*step.y_range.end(),
									z_range: *step.z_range.start()..=z,
								},
								RebootStep {
									on: step.on,
									x_range: step.x_range.clone(),
									y_range: *step.y_range.start()..=y,
									z_range: z..=*step.z_range.end(),
								},
								RebootStep {
									on: step.on,
									x_range: step.x_range.clone(),
									y_range: y..=*step.y_range.end(),
									z_range: z..=*step.z_range.end(),
								},
							]
						}
					}
					Split::Y { x, z } => {
						if !step.x_range.contains(&x) || !step.z_range.contains(&z) {
							vec![step]
						} else {
							vec![
								RebootStep {
									on: step.on,
									x_range: *step.x_range.start()..=x,
									y_range: step.y_range.clone(),
									z_range: *step.z_range.start()..=z,
								},
								RebootStep {
									on: step.on,
									x_range: x..=*step.x_range.end(),
									y_range: step.y_range.clone(),
									z_range: *step.z_range.start()..=z,
								},
								RebootStep {
									on: step.on,
									x_range: *step.x_range.start()..=x,
									y_range: step.y_range.clone(),
									z_range: z..=*step.z_range.end(),
								},
								RebootStep {
									on: step.on,
									x_range: x..=*step.y_range.end(),
									y_range: step.y_range.clone(),
									z_range: z..=*step.z_range.end(),
								},
							]
						}
					}
					Split::Z { x, y } => {
						if !step.x_range.contains(&x) || !step.y_range.contains(&y) {
							vec![step]
						} else {
							vec![
								RebootStep {
									on: step.on,
									x_range: *step.x_range.start()..=x,
									y_range: *step.y_range.start()..=y,
									z_range: step.z_range.clone(),
								},
								RebootStep {
									on: step.on,
									x_range: *step.x_range.start()..=x,
									y_range: y..=*step.y_range.end(),
									z_range: step.z_range.clone(),
								},
								RebootStep {
									on: step.on,
									x_range: x..=*step.x_range.end(),
									y_range: *step.y_range.start()..=y,
									z_range: step.z_range.clone(),
								},
								RebootStep {
									on: step.on,
									x_range: x..=*step.x_range.end(),
									y_range: y..=*step.y_range.end(),
									z_range: step.z_range.clone(),
								},
							]
						}
					}
				})
				.filter(|step| {
					// we do not need anything that intersects with a later one
					!step.intersects_exclusive(other)
						&& !step.x_range.is_empty()
						&& !step.y_range.is_empty()
						&& !step.z_range.is_empty()
				})
				.collect()
		}

		println!("Step created {} splinters", step_splinters.len());

		step_splinters
	}

	fn get_split_lines(&self) -> Vec<Split> {
		let RebootStep {
			x_range,
			y_range,
			z_range,
			..
		} = self;

		vec![
			Split::X {
				y: *y_range.start(),
				z: *z_range.start(),
			},
			Split::X {
				y: *y_range.end(),
				z: *z_range.start(),
			},
			Split::X {
				y: *y_range.start(),
				z: *z_range.end(),
			},
			Split::X {
				y: *y_range.end(),
				z: *z_range.end(),
			},
			Split::Y {
				x: *x_range.start(),
				z: *z_range.start(),
			},
			Split::Y {
				x: *x_range.end(),
				z: *z_range.start(),
			},
			Split::Y {
				x: *x_range.start(),
				z: *z_range.end(),
			},
			Split::Y {
				x: *x_range.end(),
				z: *z_range.end(),
			},
			Split::Z {
				x: *x_range.start(),
				y: *y_range.start(),
			},
			Split::Z {
				x: *x_range.start(),
				y: *y_range.end(),
			},
			Split::Z {
				x: *x_range.end(),
				y: *y_range.start(),
			},
			Split::Z {
				x: *x_range.end(),
				y: *y_range.end(),
			},
		]
	}
}

fn parse_steps(s: &str) -> (Vec<RebootStep>, Vec<RebootStep>) {
	let all_steps: Vec<_> = s
		.lines()
		.map(|line| {
			let (task, rest) = line.split_once(" x=").unwrap();
			let on = task == "on";

			let (x_start, rest) = rest.split_once("..").unwrap();
			let x_start = x_start.parse::<i32>().unwrap();

			let (x_end, rest) = rest.split_once(",y=").unwrap();
			let x_end = x_end.parse::<i32>().unwrap();

			let (y_start, rest) = rest.split_once("..").unwrap();
			let y_start = y_start.parse::<i32>().unwrap();

			let (y_end, rest) = rest.split_once(",z=").unwrap();
			let y_end = y_end.parse::<i32>().unwrap();

			let (z_start, z_end) = rest.split_once("..").unwrap();
			let z_start = z_start.parse::<i32>().unwrap();
			let z_end = z_end.parse::<i32>().unwrap();

			let x_range = x_start..=x_end;
			let y_range = y_start..=y_end;
			let z_range = z_start..=z_end;

			RebootStep {
				on,
				x_range,
				y_range,
				z_range,
			}
		})
		.collect();

	let steps_50 = all_steps
		.iter()
		.filter_map(|step| {
			let x_range = (*step.x_range.start()).max(-50)..=(*step.x_range.end()).min(50);
			let y_range = (*step.y_range.start()).max(-50)..=(*step.y_range.end()).min(50);
			let z_range = (*step.z_range.start()).max(-50)..=(*step.z_range.end()).min(50);
			if !x_range.is_empty() && !y_range.is_empty() && !z_range.is_empty() {
				Some(RebootStep {
					on: step.on,
					x_range,
					y_range,
					z_range,
				})
			} else {
				None
			}
		})
		.collect();

	(all_steps, steps_50)
}

pub fn solve(input: &str) -> Solution {
	let (steps, _) = parse_steps(input);

	// let mut count_50 = 0;

	let splintered_steps =
		steps
			.into_iter()
			.fold::<Vec<RebootStep>, _>(Vec::new(), |mut steps, next_step| {
				if steps.is_empty() {
					steps.push(next_step);
					steps
				} else {
					steps
						.into_iter()
						.flat_map(|step| {
							if step.intersects_exclusive(&next_step) {
								step.splinter(&next_step)
							} else {
								vec![step]
							}
						})
						.collect()
				}
			});

	// for x in -50..50 {
	// 	for y in -50..=50 {
	// 		for z in -50..=50 {
	// 			if let Some(task) = steps_50.iter().rfind(|step| {
	// 				step.x_range.contains(&x)
	// 					&& step.y_range.contains(&y)
	// 					&& step.z_range.contains(&z)
	// 			}) {
	// 				count_50 += task.on as u64;
	// 			}
	// 		}
	// 	}
	// }

	Solution {
		cubes_on_in_center: splintered_steps.len() as u64,
		cubes_on: 0,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let mini_example = r#"
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
"#;
		assert_eq!(solve(mini_example.trim()).cubes_on_in_center, 39);

		let input = fs::read_to_string("assets/22_sample.txt").unwrap();

		assert_eq!(solve(&input).cubes_on_in_center, 590784);
	}

	// I decided to stop at this point in 2021 because I got frustrated and
	// didn't have enough time :(

	// #[test]
	// fn part_1_solution() {
	// 	unimplemented!();
	// 	// let input = fs::read_to_string("assets/22.txt").unwrap();

	// 	// assert_eq!(solve(&input).cubes_on_in_center, 587785);
	// }

	// #[test]
	// fn part_2_example_cases() {
	// 	unimplemented!();
	// }

	// #[test]
	// fn part_2_solution() {
	// 	unimplemented!();
	// }
}
