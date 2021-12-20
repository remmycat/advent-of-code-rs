use std::collections::VecDeque;

pub struct Solution {
	lit_pixels_2: usize,
	lit_pixels_50: usize,
}

type Pixels = VecDeque<VecDeque<bool>>;

#[derive(Clone)]
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

	// #[inline(never)]
	fn get_pixel_sum(&self, x: usize, y: usize) -> usize {
		let Self { pixels, .. } = self;

		let top_row = &pixels[y - 1];
		let mid_row = &pixels[y];
		let bot_row = &pixels[y + 1];

		(take_3_around(top_row, x) << 6)
			| (take_3_around(mid_row, x) << 3)
			| take_3_around(bot_row, x)
	}

	// #[inline(never)]
	fn pad_twice(&mut self, fill: bool) {
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
		self.pad_twice(fill);

		let mut new_pixels = Vec::new();

		for y in 1..=(self.height - 2) {
			let mut new_row = Vec::new();
			for x in 1..=(self.width - 2) {
				//let is_center = y == self.width / 2 && x == self.width / 2;
				let pixel_sum = self.get_pixel_sum(x, y);
				new_row.push(pixel_lookup[pixel_sum]);
				// let px_insert = pixel_lookup[pixel_sum];
				// // print!("{}", if px_insert > 0 { '#' } else { '.' });
				// new_row |= (px_insert as u128) << (self.width - 2 - x);
			}
			// println!();
			new_pixels.push(VecDeque::from(new_row));
		}
		//

		*self = Self {
			width: self.width - 2,
			height: self.height - 2,
			pixels: VecDeque::from(new_pixels),
		};

		assert_eq!(self.pixels.len(), self.height);
	}

	fn lit_pixels(&self) -> usize {
		self.pixels
			.iter()
			.map(|row| row.iter().filter(|p| **p).count())
			.sum()
	}
}

// #[inline(never)]
fn take_3_around(deque: &VecDeque<bool>, x: usize) -> usize {
	((*deque.get(x - 1).unwrap() as usize) << 2)
		| ((*deque.get(x).unwrap() as usize) << 1)
		| (*deque.get(x + 1).unwrap() as usize)
}

// fn get_pixel_sum(image: &Image, x: usize, y: usize) -> usize {
// 	let Image { pixels, .. } = image;

// 	let top_row = &pixels[y - 1];
// 	let mid_row = &pixels[y];
// 	let bot_row = &pixels[y + 1];

// 	// ppppppppppp_lllllllllll_ttt_rrrrrrrrrr
// 	// p + l + t + r = 128;
// 	//     l + t + r = width
// 	// p             = 128 - width
// 	//         t     = 3
// 	//     l + t     = x + 1 + 1
// 	//     l         = (l + t) - t    = (x + 2) - 1    = x - 1
// 	// p + l         = (128 - width) + (x - 1)

// 	// let start_index = x - 1;
// 	// let end_index = x + 1;

// 	// let up_3 = (*up_row.get(x-1).unwrap() as u8 << 2) | *up_row.get(x-1).unwrap()

// 	// //let up_3 = (((up_row << (128 - width + x - 1)) >> (128 - 3)) as usize) << 6;
// 	// let mid_3 = (((mid_row << (128 - width + x - 1)) >> (128 - 3)) as usize) << 3;
// 	// let down_3 = ((down_row << (128 - width + x - 1)) >> (128 - 3)) as usize;

// 	// if debug {
// 	// 	println!();
// 	// 	println!();
// 	// 	println!("{:0>9b}", up_3);
// 	// 	println!("{:0>9b}", mid_3);
// 	// 	println!("{:0>9b}", down_3);
// 	// 	println!();
// 	// }

// 	(take_3_around(top_row, x) << 6) | (take_3_around(mid_row, x) << 3) | take_3_around(bot_row, x)
// }

// fn print_binary(image: &Image, prompt: String) {
// 	println!("{}:", prompt);
// 	println!();
// 	for row in image.pixels.iter() {
// 		for px in row {
// 			print!("{}", if *px { '#' } else { '.' })
// 		}
// 		println!();
// 	}
// 	println!();
// }

// fn pad_twice(image: &mut Image, fill: bool) {
// 	assert!(image.width < 124 && image.height < 124);

// 	image.pixels.iter_mut().for_each(|row| {
// 		row.push_front(fill);
// 		row.push_front(fill);
// 		row.push_back(fill);
// 		row.push_back(fill);
// 	});
// 	image.width += 4;

// 	let full_row_padding = VecDeque::from(vec![fill; image.width]);
// 	image.pixels.push_front(full_row_padding.to_owned());
// 	image.pixels.push_front(full_row_padding.to_owned());
// 	image.pixels.push_back(full_row_padding.to_owned());
// 	image.pixels.push_back(full_row_padding);

// 	image.height += 4;

// 	assert_eq!(image.pixels.len(), image.height);
// }

// fn enhance(pixel_lookup: &[bool], image: &mut Image, fill: bool) {
// 	// print_binary(
// 	// 	image,
// 	// 	format!("\nold before padding ({}x{})", image.width, image.height),
// 	// );

// 	pad_twice(image, fill);

// 	print_binary(
// 		image,
// 		format!("\nold after padding ({}x{})", image.width, image.height),
// 	);

// 	let mut new_pixels = Vec::new();

// 	for y in 1..=(image.height - 2) {
// 		let mut new_row = Vec::new();
// 		for x in 1..=(image.width - 2) {
// 			//let is_center = y == image.width / 2 && x == image.width / 2;
// 			let pixel_sum = get_pixel_sum(image, x, y);
// 			new_row.push(pixel_lookup[pixel_sum]);
// 			// let px_insert = pixel_lookup[pixel_sum];
// 			// // print!("{}", if px_insert > 0 { '#' } else { '.' });
// 			// new_row |= (px_insert as u128) << (image.width - 2 - x);
// 		}
// 		// println!();
// 		new_pixels.push(VecDeque::from(new_row));
// 	}
// 	//

// 	*image = Image {
// 		width: image.width - 2,
// 		height: image.height - 2,
// 		pixels: VecDeque::from(new_pixels),
// 	};
// 	print_binary(
// 		image,
// 		format!("\nafter enhance ({}x{})", image.width, image.height),
// 	);
// 	assert_eq!(image.pixels.len(), image.height);
// }

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

	// println!("\nstart {:#?}", start_pixels);

	let mut image = Image {
		height: start_pixels.len(),
		pixels: start_pixels,
		width,
	};

	image.enhance(&lookup, false);
	image.enhance(&lookup, even_void);

	let lit_pixels_2 = image.lit_pixels();

	// image.print("After 2".to_string());

	for i in 1..=48 {
		let void = if i % 2 == 0 { even_void } else { odd_void };
		image.enhance(&lookup, void);
	}

	let lit_pixels_50 = image.lit_pixels();
	// image.print("After 50".to_string());

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
