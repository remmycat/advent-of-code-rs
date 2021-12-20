use std::ops::{Add, AddAssign, Sub};

use hashbrown::HashSet;
use itertools::Itertools;

pub struct Solution {
	beacons: usize,
	biggest_scanner_distance: usize,
}

type Coordinate = i32;
type CoordinateScalar = i32; // for less casting

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Point {
	x: Coordinate,
	y: Coordinate,
	z: Coordinate,
}

// Please label your Matrix, Neo
struct Rotation {
	to_x: AxisRotation,
	to_y: AxisRotation,
	to_z: AxisRotation,
}
struct AxisRotation {
	from_x: CoordinateScalar,
	from_y: CoordinateScalar,
	from_z: CoordinateScalar,
}

impl AxisRotation {
	// had to do this because Default::default() is not const.
	const fn new() -> Self {
		AxisRotation {
			from_x: 0,
			from_y: 0,
			from_z: 0,
		}
	}
}

const X_POS: AxisRotation = AxisRotation {
	from_x: 1,
	..AxisRotation::new()
};
const Y_POS: AxisRotation = AxisRotation {
	from_y: 1,
	..AxisRotation::new()
};
const Z_POS: AxisRotation = AxisRotation {
	from_z: 1,
	..AxisRotation::new()
};
const X_NEG: AxisRotation = AxisRotation {
	from_x: -1,
	..AxisRotation::new()
};
const Y_NEG: AxisRotation = AxisRotation {
	from_y: -1,
	..AxisRotation::new()
};
const Z_NEG: AxisRotation = AxisRotation {
	from_z: -1,
	..AxisRotation::new()
};

// All possible 90 degree rotations
// I got annoyed by the maths and just took a die and wrote them down
#[rustfmt::skip]
const ROTATIONS: [Rotation; 24] = [
	// Z+ <= Z+
	Rotation { to_x: X_POS, to_y: Y_POS, to_z: Z_POS },
	Rotation { to_x: Y_POS, to_y: X_NEG, to_z: Z_POS },
	Rotation { to_x: X_NEG, to_y: Y_NEG, to_z: Z_POS },
	Rotation { to_x: Y_NEG, to_y: X_POS, to_z: Z_POS },
	// Z+ <= Z-
	Rotation { to_x: Y_POS, to_y: X_POS, to_z: Z_NEG },
	Rotation { to_x: X_POS, to_y: Y_NEG, to_z: Z_NEG },
	Rotation { to_x: Y_NEG, to_y: X_NEG, to_z: Z_NEG },
	Rotation { to_x: X_NEG, to_y: Y_POS, to_z: Z_NEG },
	// Z+ <= X+
	Rotation { to_x: Y_POS, to_y: Z_POS, to_z: X_POS },
	Rotation { to_x: Z_POS, to_y: Y_NEG, to_z: X_POS },
	Rotation { to_x: Y_NEG, to_y: Z_NEG, to_z: X_POS },
	Rotation { to_x: Z_NEG, to_y: Y_POS, to_z: X_POS },
	// Z+ <= X-
	Rotation { to_x: Z_POS, to_y: Y_POS, to_z: X_NEG },
	Rotation { to_x: Y_POS, to_y: Z_NEG, to_z: X_NEG },
	Rotation { to_x: Z_NEG, to_y: Y_NEG, to_z: X_NEG },
	Rotation { to_x: Y_NEG, to_y: Z_POS, to_z: X_NEG },
	// Z+ <= Y+
	Rotation { to_x: Z_POS, to_y: X_POS, to_z: Y_POS },
	Rotation { to_x: X_POS, to_y: Z_NEG, to_z: Y_POS },
	Rotation { to_x: Z_NEG, to_y: X_NEG, to_z: Y_POS },
	Rotation { to_x: X_NEG, to_y: Z_POS, to_z: Y_POS },
	// Z+ <= Y-
	Rotation { to_x: X_POS, to_y: Z_POS, to_z: Y_NEG },
	Rotation { to_x: Z_POS, to_y: X_NEG, to_z: Y_NEG },
	Rotation { to_x: X_NEG, to_y: Z_NEG, to_z: Y_NEG },
	Rotation { to_x: Z_NEG, to_y: X_POS, to_z: Y_NEG },
];

impl Point {
	fn rotate_in_place(&mut self, rot: &Rotation) {
		let Point { x, y, z } = self;
		let (x, y, z) = (*x, *y, *z);
		self.x = x * rot.to_x.from_x + y * rot.to_x.from_y + z * rot.to_x.from_z;
		self.y = x * rot.to_y.from_x + y * rot.to_y.from_y + z * rot.to_y.from_z;
		self.z = x * rot.to_z.from_x + y * rot.to_z.from_y + z * rot.to_z.from_z;
	}

	fn rotate(&self, rot: &Rotation) -> Point {
		let mut out = self.to_owned();
		out.rotate_in_place(rot);
		out
	}

	fn translate_in_place(&mut self, translation: Point) {
		*self += translation
	}

	fn translate(&self, translation: Point) -> Point {
		let mut out = self.to_owned();
		out.translate_in_place(translation);
		out
	}

	fn get_translation_to_origin(&self, origin: Point) -> Point {
		origin - self.to_owned()
	}

	fn abs(self) -> usize {
		let Point { x, y, z } = self;
		(x.abs() + y.abs() + z.abs()) as usize
	}
}

impl Add for Point {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl AddAssign for Point {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl Sub for Point {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}

pub fn solve(input: &str) -> Solution {
	let mut scanners: Vec<_> = input
		.split("\n\n")
		.map(|scanner| -> Vec<_> {
			let (_, points) = scanner.split_once("\n").unwrap();
			points
				.lines()
				.map(|line| {
					let mut coords = line.split(',');
					Point {
						x: coords.next().unwrap().parse::<Coordinate>().unwrap(),
						y: coords.next().unwrap().parse::<Coordinate>().unwrap(),
						z: coords.next().unwrap().parse::<Coordinate>().unwrap(),
					}
				})
				.collect()
		})
		.enumerate()
		.collect();

	let mut unsearched_aligned_scanners = vec![scanners.swap_remove(0)];
	let mut this_search_scanners = Vec::new();
	let mut aligned_scanners = Vec::new();
	let mut scanner_positions: Vec<Point> = vec![Point { x: 0, y: 0, z: 0 }];

	while !scanners.is_empty() {
		this_search_scanners.append(&mut unsearched_aligned_scanners);

		let mut found: Vec<usize> = Vec::new();
		'find_next_compatible: for (scanner_i, scanner) in scanners.iter() {
			for rotation in ROTATIONS.iter() {
				let rotated_scanner: Vec<_> = scanner.iter().map(|p| p.rotate(rotation)).collect();
				for (_, aligned_scanner) in this_search_scanners.iter() {
					for aligned_point in aligned_scanner {
						for rotated_point in rotated_scanner.iter() {
							let translation =
								rotated_point.get_translation_to_origin(aligned_point.to_owned());

							let translated: Vec<_> = rotated_scanner
								.iter()
								.map(|p| p.translate(translation.to_owned()))
								.collect();

							let matched = translated
								.iter()
								.filter(|p| aligned_scanner.iter().any(|a| *a == **p))
								.count();

							if matched >= 12 {
								let translated: Vec<_> = rotated_scanner
									.into_iter()
									.map(|rotated| rotated.translate(translation.to_owned()))
									.collect();
								scanner_positions.push(translation);
								unsearched_aligned_scanners.push((*scanner_i, translated));
								found.push(*scanner_i);
								continue 'find_next_compatible;
							}
						}
					}
				}
			}
		}

		aligned_scanners.append(&mut this_search_scanners);

		if found.is_empty() {
			panic!("Could not connect any more scanners to the system");
		} else {
			scanners = scanners
				.into_iter()
				.filter(|(i, _)| !found.contains(i))
				.collect();
		}
	}

	aligned_scanners.append(&mut unsearched_aligned_scanners);

	let unique_beacons: HashSet<Point> = aligned_scanners
		.into_iter()
		.flat_map(|(_, scanner)| scanner)
		.collect();

	let biggest_scanner_distance = scanner_positions
		.into_iter()
		.tuple_combinations()
		.map(|(a, b)| (a - b).abs())
		.max()
		.unwrap();

	Solution {
		beacons: unique_beacons.len(),
		biggest_scanner_distance,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let input = fs::read_to_string("assets/2021/input_19_sample.txt").unwrap();

		assert_eq!(solve(&input).beacons, 79);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_19.txt").unwrap();

		assert_eq!(solve(&input).beacons, 451);
	}

	#[test]
	fn part_2_example_cases() {
		let input = fs::read_to_string("assets/2021/input_19_sample.txt").unwrap();

		assert_eq!(solve(&input).biggest_scanner_distance, 3621);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_19.txt").unwrap();

		assert_eq!(solve(&input).biggest_scanner_distance, 0);
	}
}
