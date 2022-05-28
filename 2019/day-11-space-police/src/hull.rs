use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point {
	pub x: i16,
	pub y: i16,
}

#[derive(Debug, Clone)]
pub enum Color {
	Black,
	White,
}

impl Display for Color {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Color::Black => write!(f, "⬛")?,
			Color::White => write!(f, "⬜️")?,
		};

		Ok(())
	}
}

pub struct HullMap {
	top_left: Point,
	bottom_right: Point,
	map: HashMap<Point, Color>,
}

impl HullMap {
	pub fn new() -> Self {
		HullMap {
			map: HashMap::new(),
			top_left: Point { x: 0, y: 0 },
			bottom_right: Point { x: 0, y: 0 },
		}
	}

	pub fn get_color(&self, at: &Point) -> Color {
		self.map.get(at).cloned().unwrap_or(Color::Black)
	}

	pub fn paint(&mut self, at: Point, color: Color) {
		self.top_left.x = self.top_left.x.min(at.x);
		self.top_left.y = self.top_left.y.min(at.y);
		self.bottom_right.x = self.bottom_right.x.max(at.x);
		self.bottom_right.y = self.bottom_right.y.max(at.y);

		self.map.insert(at, color);
	}

	pub fn painted_len(&self) -> usize {
		self.map.len()
	}
}

impl Display for HullMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in self.top_left.y..=self.bottom_right.y {
			for x in self.top_left.x..=self.bottom_right.x {
				let color = self.get_color(&Point { x, y });
				write!(f, "{color}")?;
			}
			writeln!(f)?;
		}

		Ok(())
	}
}
