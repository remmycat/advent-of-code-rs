use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, String);

const NUMBERS_START: u8 = b'0';
fn parse_usize(b: &[u8]) -> usize {
	let mut num = (b[0] - NUMBERS_START) as usize;

	for digit in &b[1..] {
		if !digit.is_ascii_digit() {
			return num;
		}
		num = num * 10 + (digit - NUMBERS_START) as usize;
	}

	num
}

#[derive(Debug)]
struct Screen {
	cycle: usize,
	sprite_x: isize,
	pixels: [[bool; 40]; 6],
	signal_strengths: usize,
}

impl Screen {
	fn new() -> Self {
		Screen {
			cycle: 0,
			sprite_x: 1,
			pixels: [[false; 40]; 6],
			signal_strengths: 0,
		}
	}

	fn sprite_range(&self) -> RangeInclusive<isize> {
		(self.sprite_x - 1)..=(self.sprite_x + 1)
	}

	fn check_pixels(&mut self) {
		let pos = self.cycle - 1;
		let pos_x = pos % 40;
		let pos_y = pos / 40;

		if self.sprite_range().contains(&(pos_x as isize)) {
			self.pixels[pos_y][pos_x] = true;
		}
	}

	fn check_signal_strengths(&mut self) {
		if self.cycle % 40 == 20 {
			self.signal_strengths += (self.sprite_x as usize) * self.cycle;
		}
	}

	fn advance(&mut self) {
		self.cycle += 1;
		self.check_pixels();
		self.check_signal_strengths();
	}

	fn x_add(&mut self, inc: usize) {
		self.advance();
		self.advance();
		self.sprite_x += inc as isize;
	}

	fn x_sub(&mut self, dec: usize) {
		self.advance();
		self.advance();
		self.sprite_x -= dec as isize;
	}

	fn output(&self) -> String {
		let lines: Vec<String> = self
			.pixels
			.iter()
			.map(|line| {
				line.iter()
					.map(|i| if *i { '█' } else { '┄' })
					.collect::<String>()
			})
			.collect();

		lines.join("\n")
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let mut screen: Screen = Screen::new();

	for line in input.split(|b| *b == b'\n') {
		if line.len() == 4 {
			// noop
			screen.advance()
		} else if line[5] == b'-' {
			screen.x_sub(parse_usize(&line[6..]))
		} else {
			screen.x_add(parse_usize(&line[5..]))
		}
	}

	screen.advance();

	Solution(screen.signal_strengths, screen.output())
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	const EXAMPLE_SCREEN: &str = r#"
██┄┄██┄┄██┄┄██┄┄██┄┄██┄┄██┄┄██┄┄██┄┄██┄┄
███┄┄┄███┄┄┄███┄┄┄███┄┄┄███┄┄┄███┄┄┄███┄
████┄┄┄┄████┄┄┄┄████┄┄┄┄████┄┄┄┄████┄┄┄┄
█████┄┄┄┄┄█████┄┄┄┄┄█████┄┄┄┄┄█████┄┄┄┄┄
██████┄┄┄┄┄┄██████┄┄┄┄┄┄██████┄┄┄┄┄┄████
███████┄┄┄┄┄┄┄███████┄┄┄┄┄┄┄███████┄┄┄┄┄"#;

	const SOLUTION_SCREEN: &str = r#"
████┄┄██┄┄█┄┄┄┄█┄┄█┄███┄┄█┄┄┄┄████┄┄┄██┄
█┄┄┄┄█┄┄█┄█┄┄┄┄█┄┄█┄█┄┄█┄█┄┄┄┄█┄┄┄┄┄┄┄█┄
███┄┄█┄┄┄┄█┄┄┄┄████┄███┄┄█┄┄┄┄███┄┄┄┄┄█┄
█┄┄┄┄█┄██┄█┄┄┄┄█┄┄█┄█┄┄█┄█┄┄┄┄█┄┄┄┄┄┄┄█┄
█┄┄┄┄█┄┄█┄█┄┄┄┄█┄┄█┄█┄┄█┄█┄┄┄┄█┄┄┄┄█┄┄█┄
████┄┄███┄████┄█┄┄█┄███┄┄████┄█┄┄┄┄┄██┄┄"#;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(13140, EXAMPLE_SCREEN.trim().to_string()))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(13920, SOLUTION_SCREEN.trim().to_string()))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
