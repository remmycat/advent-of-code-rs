use hashbrown::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

// Rocks are round, stones are cubic. Obviously.
const ROCK: u8 = b'O';
const STONE: u8 = b'#';

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Solid {
	Rock,
	Stone,
	Nothing,
}

impl Solid {
	fn parse_u8(b: &u8) -> Self {
		match *b {
			ROCK => Self::Rock,
			STONE => Self::Stone,
			_ => Self::Nothing,
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
	width: usize,
	height: usize,
	solids: Vec<Solid>,
}

// impl fmt::Debug for Grid {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		write!(f, "Grid ({} x {})", self.width, self.height)?;
// 		write!(f, "\n ┏")?;
// 		for _ in 0..self.width {
// 			write!(f, "━")?;
// 		}
// 		write!(f, "┓")?;
// 		for y in 0..self.height {
// 			write!(f, "\n ┃")?;
// 			let x_start = y * self.width;
// 			for solid in self.solids[x_start..(x_start + self.width)].iter() {
// 				match solid {
// 					Solid::Rock => write!(f, "○"),
// 					Solid::Stone => write!(f, "▇"),
// 					_ => write!(f, " "),
// 				}?;
// 			}
// 			write!(f, "┃")?;
// 		}
// 		write!(f, "\n ┗")?;
// 		for _ in 0..self.width {
// 			write!(f, "━")?;
// 		}
// 		write!(f, "┛")?;
// 		Ok(())
// 	}
// }

impl Grid {
	fn parse(input: &[u8]) -> Self {
		let width = input
			.iter()
			.position(|b| *b == b'\n')
			.expect("input must have a newline");
		let line_width = width + 1;
		let height = input.len() / line_width;

		let solids: Vec<Solid> = input
			.iter()
			.filter(|b| **b != b'\n')
			.map(Solid::parse_u8)
			.collect();

		Self {
			width,
			height,
			solids,
		}
	}

	fn up(&mut self) {
		for x in 0..self.width {
			let mut spot = x;
			for y in 0..self.height {
				let index = y * self.width + x;
				match self.solids[index] {
					Solid::Rock => {
						self.solids.swap(spot, index);
						spot += self.width;
					}
					Solid::Stone => {
						spot = index + self.width;
					}
					_ => (),
				};
			}
		}
	}

	fn left(&mut self) {
		for y in 0..self.height {
			let mut spot = y * self.width;
			for x in 0..self.width {
				let index = y * self.width + x;
				match self.solids[index] {
					Solid::Rock => {
						self.solids.swap(spot, index);
						spot += 1;
					}
					Solid::Stone => {
						spot = index + 1;
					}
					_ => (),
				};
			}
		}
	}

	fn down(&mut self) {
		for x in 0..self.width {
			let mut spot = self.width * (self.height - 1) + x;
			for y in (0..self.height).rev() {
				let index = y * self.width + x;
				match self.solids[index] {
					Solid::Rock => {
						self.solids.swap(spot, index);
						spot = spot.checked_sub(self.width).unwrap_or(spot);
					}
					Solid::Stone => {
						spot = index.checked_sub(self.width).unwrap_or(spot);
					}
					_ => (),
				};
			}
		}
	}

	fn right(&mut self) {
		for y in 0..self.height {
			let mut spot = (y + 1) * self.width - 1;
			for x in (0..self.width).rev() {
				let index = y * self.width + x;
				match self.solids[index] {
					Solid::Rock => {
						self.solids.swap(spot, index);
						spot = spot.checked_sub(1).unwrap_or(spot);
					}
					Solid::Stone => {
						spot = index.checked_sub(1).unwrap_or(spot);
					}
					_ => (),
				};
			}
		}
	}

	fn cycle(&mut self) {
		self.up();
		self.left();
		self.down();
		self.right();
	}

	fn get_a_load_of_these_solids(&self) -> usize {
		self.solids
			.iter()
			.enumerate()
			.map(|(index, solid)| match solid {
				Solid::Rock => self.height - (index / self.width),
				_ => 0,
			})
			.sum()
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let mut grid = Grid::parse(input);

	// 1 manual cycle for part 1
	grid.up();
	let load = grid.get_a_load_of_these_solids();
	grid.left();
	grid.down();
	grid.right();

	let mut memo: HashMap<Grid, usize> = HashMap::new();
	let mut remaining: usize = 0;

	for i in 1..1_000_000_000 {
		grid.cycle();
		if let Some(last_i) = memo.get(&grid) {
			let diff = i - last_i;
			let left_i = 1_000_000_000 - i - 1;
			remaining = left_i % diff;
			break;
		} else {
			memo.insert(grid.clone(), i);
		}
	}

	for _ in 0..remaining {
		grid.cycle();
	}

	let end_load = grid.get_a_load_of_these_solids();

	Solution(load, end_load)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(136,64),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(108614,96447),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
