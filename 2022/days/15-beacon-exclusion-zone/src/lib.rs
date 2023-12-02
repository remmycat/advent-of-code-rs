use aoc_2022_utils::ascii_int::parse_int;

use crate::range_set::IntRangeSet;

mod range_set;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
	x: isize,
	y: isize,
}

// 45° clockwise (ish)
// x and y are now on a diagonal grid that let's us treat rhombi as squares
#[derive(Debug, Clone)]
struct DiagonalPoint {
	x: isize,
	y: isize,
}

impl Point {
	fn distance_to(&self, other: &Point) -> usize {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
	}

	fn get_tuning_frequency(&self) -> usize {
		self.x as usize * 4_000_000 + self.y as usize
	}
}

impl From<Point> for DiagonalPoint {
	fn from(point: Point) -> Self {
		DiagonalPoint {
			x: point.x + point.y,
			y: -point.x + point.y,
		}
	}
}

impl From<DiagonalPoint> for Point {
	fn from(diag: DiagonalPoint) -> Self {
		Point {
			x: (diag.x - diag.y) / 2,
			y: (diag.x + diag.y) / 2,
		}
	}
}

#[derive(Debug)]
struct Rhombus {
	/// since the diagonal point is in the rotated system this is the top left
	/// of the sqare (x_min, y_min)
	top: DiagonalPoint,
	/// bottom right in the diagonal system's square (x_max, y_max)
	bottom: DiagonalPoint,
}

impl Rhombus {
	fn corners(self) -> [Point; 4] {
		[
			DiagonalPoint {
				x: self.top.x,
				y: self.bottom.y,
			}
			.into(),
			DiagonalPoint {
				x: self.bottom.x,
				y: self.top.y,
			}
			.into(),
			self.top.into(),
			self.bottom.into(),
		]
	}

	fn edge_intersection_corners(&self, other: &Rhombus) -> Option<[Point; 4]> {
		// we're basically intersecting not the rhombi, but their outer edges
		// because we know the point we're looking for is in one such
		// intersection
		let our_x_start = self.top.x - 1;
		let our_x_end = self.bottom.x + 1;
		let their_x_start = other.top.x - 1;
		let their_x_end = other.bottom.x + 1;
		let our_y_start = self.top.y - 1;
		let our_y_end = self.bottom.y + 1;
		let their_y_start = other.top.y - 1;
		let their_y_end = other.bottom.y + 1;

		if our_x_end < their_x_start
			|| our_x_start > their_x_end
			|| our_y_end < their_y_start
			|| our_y_start > their_y_end
		{
			// Not touching or intersecting
			None
		} else {
			Some(
				Rhombus {
					top: DiagonalPoint {
						x: our_x_start.max(their_x_start),
						y: our_y_start.max(their_y_start),
					},
					bottom: DiagonalPoint {
						x: our_y_start.min(their_y_start),
						y: our_y_end.min(their_y_end),
					},
				}
				.corners(),
			)
		}
	}
}

#[derive(Debug)]
struct Sensor {
	location: Point,
	beacon: Point,
	exclusion_distance: usize,
}

impl Sensor {
	fn get_excluded_ranges_at_y(&self, y: isize) -> Vec<(isize, isize)> {
		let x_diff = (self.exclusion_distance - self.location.y.abs_diff(y)) as isize;
		let range = (self.location.x - x_diff, self.location.x + x_diff);

		if self.beacon.y == y {
			[(range.0, self.beacon.x - 1), (self.beacon.x + 1, range.1)]
				.into_iter()
				.filter(|r| r.1 >= r.0)
				.collect()
		} else {
			vec![range]
		}
	}
}

impl From<&[u8]> for Sensor {
	fn from(value: &[u8]) -> Self {
		let mut parts = value.split(|b| matches!(b, b' ' | b'=' | b',' | b':'));

		let location = Point {
			x: parse_int(parts.nth(3).expect("Bad input, expected sensor x")),
			y: parse_int(parts.nth(2).expect("Bad input, expected sensor y")),
		};

		let beacon = Point {
			x: parse_int(parts.nth(6).expect("Bad input, expected beacon x")),
			y: parse_int(parts.nth(2).expect("Bad input, expected beacon y")),
		};

		let exclusion_distance = location.distance_to(&beacon);

		Sensor {
			location,
			beacon,
			exclusion_distance,
		}
	}
}

impl From<&Sensor> for Rhombus {
	fn from(sensor: &Sensor) -> Self {
		Rhombus {
			top: Point {
				x: sensor.location.x,
				y: sensor.location.y - sensor.exclusion_distance as isize,
			}
			.into(),
			bottom: Point {
				x: sensor.location.x,
				y: sensor.location.y + sensor.exclusion_distance as isize,
			}
			.into(),
		}
	}
}

pub fn solve(input: &[u8], y_check: isize, search_scope: isize) -> Solution {
	let sensors: Vec<_> = input.split(|b| *b == b'\n').map(Sensor::from).collect();

	assert_eq!(
		Point { x: -3, y: 15 },
		Point::from(DiagonalPoint::from(Point { x: -3, y: 15 })),
	);

	let unchecked_count = sensors
		.iter()
		.filter(|s| s.location.y.abs_diff(y_check) <= s.exclusion_distance)
		.flat_map(|s| s.get_excluded_ranges_at_y(y_check))
		.fold(IntRangeSet::new(), |mut set, range| {
			set.add_range(range);
			set
		})
		.len();

	let search_range = 0..=search_scope;

	let rhombs: Vec<Rhombus> = sensors.iter().map(Rhombus::from).collect();

	let beacon = rhombs
		.iter()
		.enumerate()
		.flat_map(|(index, rhomb)| {
			rhombs.as_slice()[index + 1..]
				.iter()
				.flat_map(|other| rhomb.edge_intersection_corners(other))
		})
		.flatten()
		.filter(|p| search_range.contains(&p.x) && search_range.contains(&p.y))
		// For completeness sake: the corner points could be missed by the intersection algorithm
		.chain([
			Point { x: 0, y: 0 },
			Point {
				x: 0,
				y: search_scope,
			},
			Point {
				x: search_scope,
				y: 0,
			},
			Point {
				x: search_scope,
				y: search_scope,
			},
		])
		.find(|p| {
			sensors
				.iter()
				.all(|s| s.location.distance_to(p) > s.exclusion_distance)
		})
		.expect("No possible beacon point found in search range?");

	let tuning_freq = beacon.get_tuning_frequency();

	Solution(unchecked_count, tuning_freq)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), 10, 20, Solution(26,56000011))]
	#[case(include_bytes!("../inputs/personal.txt"), 2_000_000, 4_000_000, Solution(5083287,13134039205729))]
	fn solution(
		#[case] input: &[u8],
		#[case] y_check: isize,
		#[case] search_scope: isize,
		#[case] expected: Solution,
	) {
		assert_eq!(solve(input, y_check, search_scope), expected);
	}
}
