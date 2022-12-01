use std::collections::HashMap;

use fallible_iterator::{FallibleIterator, IntoFallibleIterator};
use intcode::{IntCodeError, IntCodeProgram};

pub enum Tile {
	Empty,
	Wall,
	Block,
	HorizontalPaddle,
	Ball,
}

impl From<i64> for Tile {
	fn from(id: i64) -> Self {
		match id {
			0 => Tile::Empty,
			1 => Tile::Wall,
			2 => Tile::Block,
			3 => Tile::HorizontalPaddle,
			4 => Tile::Ball,
			other => panic!("unknown tile type {other}"),
		}
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point {
	pub x: i64,
	pub y: i64,
}

pub struct Screen {
	pub grid: HashMap<Point, Tile>,
}

impl Screen {
	pub fn new() -> Self {
		Self {
			grid: HashMap::new(),
		}
	}

	pub fn run(&mut self, software: IntCodeProgram) -> Result<(), IntCodeError> {
		let mut running_software = software.into_fallible_iter();

		while let Some(x) = running_software.next()? {
			let y = running_software.next()?.expect("expected y value after x");
			let tile_id = running_software.next()?.expect("expected tile id after y");

			let point = Point { x, y };
			let tile = Tile::from(tile_id);

			self.grid.insert(point, tile);
		}

		Ok(())
	}
}
