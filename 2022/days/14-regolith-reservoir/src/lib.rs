#[derive(Debug, PartialEq, Eq)]
pub struct Solution(u64, u64);

const X_OFFSET: usize = 500 - 200;
const X_500: usize = 500 - X_OFFSET;

pub fn solve(input: &[u8]) -> Solution {
	let mut map = CaveMap { lines: vec![] };

	for line in input.split(|b| *b == b'\n') {
		let mut last: Option<(usize, usize)> = None;
		for pair in line.split(|b| *b == b' ').step_by(2) {
			let x = parse_uint(&pair[0..3]) - X_OFFSET;
			let y = parse_uint(&pair[4..]);

			let Some((x_old, y_old)) = last else {
				last = Some((x, y));
				continue;
			};

			if x == x_old {
				map.fill_column(x, y_old, y)
			} else {
				debug_assert_eq!(y, y_old);

				map.fill_row(y, x_old, x);
			}
			last = Some((x, y));
		}
	}

	let sand_count = map.fill_with_sand();

	map.fill_row(map.lines.len() + 1, 0, 399);

	let sand_count_2 = sand_count + map.fill_with_sand();

	Solution(sand_count, sand_count_2)
}

fn b_digit(b: u8) -> usize {
	(b - b'0') as usize
}

fn parse_uint(digits: &[u8]) -> usize {
	digits[1..]
		.iter()
		.fold(b_digit(digits[0]), |dig, b| dig * 10 + b_digit(*b))
}

#[derive(Debug)]
struct CaveMap {
	lines: Vec<[bool; 400]>,
}

impl CaveMap {
	fn grow(&mut self, y: usize) {
		if y >= self.lines.len() {
			self.lines.resize(y + 1, [false; 400]);
		}
	}

	fn fill_row(&mut self, y: usize, xa: usize, xb: usize) {
		self.grow(y);

		let (xmin, xmax) = if xa < xb { (xa, xb) } else { (xb, xa) };

		for point in self.lines[y].iter_mut().take(xmax + 1).skip(xmin) {
			*point = true;
		}
	}

	fn fill_column(&mut self, x: usize, ya: usize, yb: usize) {
		let (ymin, ymax) = if ya < yb { (ya, yb) } else { (yb, ya) };

		self.grow(ymax);

		for line in &mut self.lines.iter_mut().take(ymax + 1).skip(ymin) {
			line[x] = true;
		}
	}

	fn fill_with_sand(&mut self) -> u64 {
		let mut count = 0;
		let mut sand_path: Vec<(usize, usize)> = vec![];
		let mut current: Option<(usize, usize)> = Some((X_500, 0));

		loop {
			let Some((x, y)) = current else {
				// max pileup
				return count
			};
			if y == self.lines.len() - 1 {
				break;
			}

			if !self.lines[y + 1][x] {
				// down is empty
				sand_path.push((x, y));
				current = Some((x, y + 1));
			} else if !self.lines[y + 1][x - 1] {
				// down left is empty
				sand_path.push((x, y));
				current = Some((x - 1, y + 1));
			} else if !self.lines[y + 1][x + 1] {
				// down right is empty
				sand_path.push((x, y));
				current = Some((x + 1, y + 1));
			} else {
				// Sand stopped, let's backtrack and look at the next grain
				count += 1;
				self.lines[y][x] = true;
				current = sand_path.pop();
			}
		}

		count
	}

	// fn print(&self) -> String {
	// 	let lines: Vec<String> = (0..self.lines.len())
	// 		.map(|y| {
	// 			(0..400)
	// 				.map(|x| if self.lines[y][x] { '#' } else { '.' })
	// 				.collect()
	// 		})
	// 		.collect();

	// 	lines.join("\n")
	// }
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(24,93))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(1199,23925))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
