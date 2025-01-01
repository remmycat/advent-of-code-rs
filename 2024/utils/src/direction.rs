use std::{
	fmt::Debug,
	ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	North,
	East,
	South,
	West,
}

pub use Direction::*;

impl Direction {
	#[inline]
	#[must_use]
	pub const fn opposite(&self) -> Direction {
		match self {
			North => South,
			East => West,
			South => North,
			West => East,
		}
	}

	#[inline]
	#[must_use]
	pub const fn turn_clockwise(&self) -> Direction {
		match self {
			North => East,
			East => South,
			South => West,
			West => North,
		}
	}

	#[inline]
	#[must_use]
	pub const fn turn_widdershins(&self) -> Direction {
		match self {
			North => West,
			East => North,
			South => East,
			West => South,
		}
	}

	#[inline]
	#[must_use]
	pub const fn turn_clockwise_times(&self, n_times: usize) -> Direction {
		match n_times % 4 {
			0 => *self,
			1 => self.turn_clockwise(),
			2 => self.opposite(),
			3 => self.turn_widdershins(),
			_ => unreachable!(),
		}
	}

	#[inline]
	#[must_use]
	pub const fn turn_widdershins_times(&self, n_times: usize) -> Direction {
		match n_times % 4 {
			0 => *self,
			1 => self.turn_widdershins(),
			2 => self.opposite(),
			3 => self.turn_clockwise(),
			_ => unreachable!(),
		}
	}
}

impl Neg for Direction {
	type Output = Direction;

	#[inline]
	fn neg(self) -> Self::Output {
		self.opposite()
	}
}

impl Add<usize> for Direction {
	type Output = Direction;

	#[inline]
	fn add(self, rhs: usize) -> Self::Output {
		self.turn_clockwise_times(rhs)
	}
}

impl AddAssign<usize> for Direction {
	#[inline]
	fn add_assign(&mut self, rhs: usize) {
		*self = self.turn_clockwise_times(rhs);
	}
}

impl Sub<usize> for Direction {
	type Output = Direction;

	#[inline]
	fn sub(self, rhs: usize) -> Self::Output {
		self.turn_widdershins_times(rhs)
	}
}

impl SubAssign<usize> for Direction {
	#[inline]
	fn sub_assign(&mut self, rhs: usize) {
		*self = self.turn_widdershins_times(rhs);
	}
}

impl Debug for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if f.alternate() {
			match self {
				North => write!(f, "N ↑"),
				East => write!(f, "E →"),
				South => write!(f, "S ↓"),
				West => write!(f, "W ←"),
			}
		} else {
			match self {
				North => write!(f, "North"),
				East => write!(f, "East"),
				South => write!(f, "South"),
				West => write!(f, "West"),
			}
		}
	}
}
