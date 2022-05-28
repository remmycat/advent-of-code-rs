use fallible_iterator::{FallibleIterator, IntoFallibleIterator};
use intcode::{IntCodeError, IntCodeProgram};

use super::hull::{Color, HullMap, Point};

enum Direction {
	Up,
	Right,
	Down,
	Left,
}

pub struct Robot {
	direction: Direction,
	pub location: Point,
}

impl Robot {
	pub fn new() -> Self {
		Robot {
			direction: Direction::Up,
			location: Point { x: 0, y: 0 },
		}
	}

	pub fn run_program(
		mut self,
		brain: IntCodeProgram,
		initial_panel: Color,
	) -> Result<HullMap, IntCodeError> {
		let mut hull = HullMap::new();

		hull.paint(self.location.clone(), initial_panel);

		// Initial input
		let brain = brain.inputs(vec![self.camera(&hull)]);

		let mut brain_process = brain.into_fallible_iter();

		while let Some(paint_color) = brain_process.next()? {
			let color = match paint_color {
				0 => Color::Black,
				1 => Color::White,
				_ => panic!("invalid color returned from hull painting program"),
			};

			hull.paint(self.location.clone(), color);

			let turn_direction = brain_process
				.next()?
				.expect("expected turn direction from hull painting program");

			match turn_direction {
				0 => self.turn_left(),
				1 => self.turn_right(),
				_ => panic!("invalid turn direction returned from hull painting program"),
			}

			brain_process.add_input(self.camera(&hull))
		}

		Ok(hull)
	}

	fn camera(&self, hull: &HullMap) -> i64 {
		match hull.get_color(&self.location) {
			Color::Black => 0,
			Color::White => 1,
		}
	}

	fn forward(&mut self) {
		match self.direction {
			Direction::Up => self.location.y -= 1,
			Direction::Right => self.location.x += 1,
			Direction::Down => self.location.y += 1,
			Direction::Left => self.location.x -= 1,
		}
	}

	fn turn_left(&mut self) {
		self.direction = match self.direction {
			Direction::Up => Direction::Left,
			Direction::Right => Direction::Up,
			Direction::Down => Direction::Right,
			Direction::Left => Direction::Down,
		};
		self.forward();
	}

	fn turn_right(&mut self) {
		self.direction = match self.direction {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		};
		self.forward();
	}
}
