use std::collections::VecDeque;

use itertools::Itertools;

pub struct Solution {
	lit_pixels_2: usize,
	lit_pixels_50: usize,
}

type Pixels = VecDeque<VecDeque<bool>>;

struct Image {
	pixels: Pixels,
	width: usize,
	height: usize,
}

impl Image {
	fn print(&self, prompt: String) {
		println!("{}:", prompt);
		println!();
		for row in self.pixels.iter() {
			for px in row {
				print!("{}", if *px { '#' } else { '.' })
			}
			println!();
		}
		println!();
	}

	fn double_pad(&mut self, fill: bool) {
		self.pixels.iter_mut().for_each(|row| {
			row.push_front(fill);
			row.push_front(fill);
			row.push_back(fill);
			row.push_back(fill);
		});
		self.width += 4;

		let full_row_padding = VecDeque::from(vec![fill; self.width]);
		self.pixels.push_front(full_row_padding.to_owned());
		self.pixels.push_front(full_row_padding.to_owned());
		self.pixels.push_back(full_row_padding.to_owned());
		self.pixels.push_back(full_row_padding);

		self.height += 4;
	}

	fn enhance(&mut self, pixel_lookup: &[bool], fill: bool) {
		self.double_pad(fill);

		let new_height = self.height - 2;
		let new_width = self.width - 2;

		self.pixels = self.pixels.iter().tuple_windows::<(_, _, _)>().fold(
			VecDeque::with_capacity(new_height),
			|mut px_acc, (top, mid, bot)| {
				let mut new_mid = VecDeque::with_capacity(new_width);

				for x in 1..=new_width {
					let p1 = (top[x - 1] as usize) << 8;
					let p2 = (top[x] as usize) << 7;
					let p3 = (top[x + 1] as usize) << 6;
					let p4 = (mid[x - 1] as usize) << 5;
					let p5 = (mid[x] as usize) << 4;
					let p6 = (mid[x + 1] as usize) << 3;
					let p7 = (bot[x - 1] as usize) << 2;
					let p8 = (bot[x] as usize) << 1;
					let p9 = bot[x + 1] as usize;

					let pixel_sum = p1 | p2 | p3 | p4 | p5 | p6 | p7 | p8 | p9;

					new_mid.push_back(pixel_lookup[pixel_sum]);
				}

				px_acc.push_back(new_mid);

				px_acc
			},
		);

		self.width = new_width;
		self.height = new_height;

		assert_eq!(self.pixels.len(), self.height);
	}

	fn lit_pixels(&self) -> usize {
		self.pixels
			.iter()
			.map(|row| row.iter().filter(|p| **p).count())
			.sum()
	}
}

pub fn solve(input: &str) -> Solution {
	let (lookup, start_pixels) = input.split_once("\n\n").unwrap();
	let lookup: Vec<_> = lookup.chars().map(|char| char == '#').collect();

	let even_void = *lookup.first().unwrap();
	let odd_void = *lookup.get(if even_void { 0b111111111 } else { 0 }).unwrap();

	let mut width = 0;
	let start_pixels: Pixels = start_pixels
		.lines()
		.map(|line| -> VecDeque<_> {
			width = line.len();
			line.chars().map(|char| char == '#').collect()
		})
		.collect();

	let mut image = Image {
		height: start_pixels.len(),
		pixels: start_pixels,
		width,
	};

	image.enhance(&lookup, false);
	image.enhance(&lookup, even_void);

	let lit_pixels_2 = image.lit_pixels();

	for i in 1..=48 {
		let void = if i % 2 == 0 { even_void } else { odd_void };
		image.enhance(&lookup, void);
	}

	let lit_pixels_50 = image.lit_pixels();

	Solution {
		lit_pixels_2,
		lit_pixels_50,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let input = fs::read_to_string("assets/2021/input_20_sample.txt").unwrap();

		assert_eq!(solve(&input).lit_pixels_2, 35);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_20.txt").unwrap();

		assert_eq!(solve(&input).lit_pixels_2, 5819);
	}

	#[test]
	fn part_2_example_cases() {
		let input = fs::read_to_string("assets/2021/input_20_sample.txt").unwrap();

		assert_eq!(solve(&input).lit_pixels_50, 3351);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_20.txt").unwrap();

		assert_eq!(solve(&input).lit_pixels_50, 18516);
	}
}
