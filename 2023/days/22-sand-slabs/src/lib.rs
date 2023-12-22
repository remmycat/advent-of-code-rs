use aoc_utils::{ascii_int::parse_uint, iteration::expect_n, trim::trim_end_newline};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug)]
struct SandSlab {
	z: (usize, usize),
	plane: Plane,
	supporting: Vec<usize>,
	supported_by: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Plane {
	x: (usize, usize),
	y: (usize, usize),
}

impl Plane {
	fn collides(&self, other: &Plane) -> bool {
		range_collision(self.x, other.x) && range_collision(self.y, other.y)
	}
}

fn range_collision<T: Ord>(a: (T, T), b: (T, T)) -> bool {
	a.0 <= b.1 && a.1 >= b.0
}

impl SandSlab {
	fn from_bytes(bytes: &[u8]) -> Self {
		let [x0, y0, z0, x1, y1, z1] = expect_n(
			bytes.split(|b| matches!(b, b',' | b'~')).map(parse_uint),
			"has 6 coordinates",
		);
		debug_assert!(x1 >= x0);
		debug_assert!(y1 >= y0);
		debug_assert!(z1 >= z0);

		SandSlab {
			plane: Plane {
				x: (x0, x1),
				y: (y0, y1),
			},
			z: (z0, z1),
			supporting: vec![],
			supported_by: vec![],
		}
	}
}

fn count_all_dependants(
	collection: &mut Vec<usize>,
	all_slabs: &[SandSlab],
	current_dependants: &[usize],
) -> usize {
	// add count of any dependant that has dependencies only on our collection!
	let mut count = 0;

	for index in current_dependants {
		let slab = &all_slabs[*index];
		if slab
			.supported_by
			.iter()
			.all(|base| collection.contains(base))
		{
			collection.push(*index);
			count += 1 + count_all_dependants(collection, all_slabs, &slab.supporting)
		}
	}

	count
}

pub fn solve(input: &[u8]) -> Solution {
	let mut slabs: Vec<_> = trim_end_newline(input)
		.split(|b| *b == b'\n')
		.map(SandSlab::from_bytes)
		.collect();

	let mut x_max: usize = 0;
	let mut y_max: usize = 0;
	let mut z_max: usize = 0;

	for slab in slabs.iter() {
		x_max = x_max.max(slab.plane.x.1);
		y_max = y_max.max(slab.plane.y.1);
		z_max = z_max.max(slab.z.1);
	}

	slabs.sort_by_key(|s| s.z.0);

	// sorted by z0 - we cannot assume max(z.1) is last slab
	let z_max = slabs
		.iter()
		.map(|s| s.z.1)
		.max()
		.expect("must have at least one slab");

	let mut settled_planes: Vec<Vec<(Plane, usize)>> = vec![vec![]; z_max];

	let mut slab_supporting_map: Vec<Vec<usize>> = vec![vec![]; slabs.len()];

	for (slab_index, slab) in slabs.iter_mut().enumerate() {
		let slab_min_z = slab.z.0;

		// find lowest z under slab.z.0 that has planar collision.
		// if we cannot find one, it will collide with the ground at z0
		let (collision_z, collisions) = (1..slab_min_z)
			.rev()
			.find_map(|z| {
				let collisions: Vec<usize> = settled_planes[z]
					.iter()
					.filter_map(|(plane, other_index)| {
						if slab.plane.collides(plane) {
							Some(*other_index)
						} else {
							None
						}
					})
					.collect();

				if !collisions.is_empty() {
					Some((z, collisions))
				} else {
					None
				}
			})
			.unwrap_or((0, vec![]));

		// move own slab.z -> z + 1 (equal would mean it's colliding!)
		let z_diff = slab_min_z - collision_z - 1;
		slab.z.0 -= z_diff;
		slab.z.1 -= z_diff;

		for other_index in collisions.iter() {
			slab_supporting_map[*other_index].push(slab_index);
		}

		slab.supported_by = collisions;

		// we only need to add "roof" of current slab to the settled planes
		settled_planes[slab.z.1].push((slab.plane.clone(), slab_index));
	}

	for (slab, supporting) in slabs.iter_mut().zip(slab_supporting_map.into_iter()) {
		slab.supporting = supporting;
	}

	let mut safely_disintegratable = 0;
	let mut chain_reaction_sum = 0;

	for (slab_index, slab) in slabs.iter().enumerate() {
		let safe = slab
			.supporting
			.iter()
			.all(|other_index| slabs[*other_index].supported_by.len() > 1);

		if safe {
			safely_disintegratable += 1;
		} else {
			let mut included_in_chain: Vec<usize> = vec![slab_index];
			chain_reaction_sum +=
				count_all_dependants(&mut included_in_chain, &slabs, &slab.supporting);
		}
	}

	Solution(safely_disintegratable, chain_reaction_sum)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(5,7),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(403,70189),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
