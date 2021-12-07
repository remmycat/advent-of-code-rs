use std::{ops::RangeInclusive, str::FromStr};

use anyhow::{bail, Error};

struct Solution {
	lights_on: u32,
	allover_brightness: u32,
}

struct Coordinates {
	x: u16,
	y: u16,
}

impl FromStr for Coordinates {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<u16> = s.split(',').map(|c| c.parse::<u16>().unwrap()).collect();
		assert_eq!(parts.len(), 2);
		Ok(Coordinates {
			x: parts[0],
			y: parts[1],
		})
	}
}

enum Task {
	TurnOn,
	TurnOff,
	Toggle,
}

struct Instruction {
	task: Task,
	x_range: RangeInclusive<u16>,
	y_range: RangeInclusive<u16>,
	y_applies: bool,
	next: Option<Box<Instruction>>,
}

impl FromStr for Instruction {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let v: Vec<_> = s.split(' ').collect();
		let (task, start, end) = match v[..] {
			["turn", "on", start, "through", end] => (Task::TurnOn, start, end),
			["turn", "off", start, "through", end] => (Task::TurnOff, start, end),
			["toggle", start, "through", end] => (Task::Toggle, start, end),
			_ => bail!("Bad input"),
		};

		let start: Coordinates = start.parse().unwrap();
		let end: Coordinates = end.parse().unwrap();

		Ok(Instruction {
			task,
			x_range: start.x..=end.x,
			y_range: start.y..=end.y,
			y_applies: true,
			next: None,
		})
	}
}

struct Light {
	x: u16,
	is_on: bool,
	brightness: u16,
}

impl Instruction {
	fn set_y(&mut self, y: &u16) {
		self.y_applies = self.y_range.contains(y);

		if self.next.is_some() {
			self.next.as_mut().unwrap().set_y(y);
		}
	}

	fn execute(&self, mut light: Light) -> Light {
		let applies = self.y_applies && self.x_range.contains(&light.x);

		if applies {
			match self.task {
				Task::TurnOn => {
					light.is_on = true;
					light.brightness += 1;
				}
				Task::TurnOff => {
					light.is_on = false;
					light.brightness = light.brightness.max(1) - 1;
				}
				Task::Toggle => {
					light.is_on = !light.is_on;
					light.brightness += 2;
				}
			}
		}
		if self.next.is_some() {
			self.next.as_ref().unwrap().execute(light)
		} else {
			light
		}
	}
}

fn solve(input: &str) -> Solution {
	let mut compiled_instructions = input
		.lines()
		.map(|line| line.parse::<Instruction>().unwrap())
		.rfold(None, |next, mut ins| {
			ins.next = next;
			Some(Box::new(ins))
		})
		.unwrap();

	let mut lights_on = 0;
	let mut allover_brightness = 0;

	for y in 0..=999 {
		compiled_instructions.set_y(&y);
		for x in 0..=999 {
			let Light {
				is_on, brightness, ..
			} = compiled_instructions.execute(Light {
				x,
				is_on: false,
				brightness: 0,
			});

			lights_on += is_on as u32;
			allover_brightness += brightness as u32;
		}
	}

	Solution {
		lights_on,
		allover_brightness,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		// Improvising a little, since the given examples weren't so good for validation
		assert_eq!(solve("turn on 0,0 through 499,999").lights_on, 500_000);
		assert_eq!(
			solve("turn on 0,0 through 499,999\ntoggle 0,0 through 999,0").lights_on,
			500_000
		);
		assert_eq!(solve("turn on 0,0 through 499,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500").lights_on, 499_998);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2015/input_06.txt").unwrap();

		assert_eq!(solve(input.trim()).lights_on, 400410);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(
			solve("turn on 0,0 through 499,999").allover_brightness,
			500_000
		);
		assert_eq!(
			solve("turn on 0,0 through 499,999\ntoggle 0,0 through 999,0").allover_brightness,
			502_000
		);
		assert_eq!(solve("turn on 0,0 through 499,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500").allover_brightness, 501_998);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2015/input_06.txt").unwrap();

		assert_eq!(solve(input.trim()).allover_brightness, 15343601);
	}
}
