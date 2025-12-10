use aoc_utils::trim::trim_end_newline;
use atoi_simd as atoi;
use itertools::Itertools;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Node(usize, usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Edge {
	indices: (usize, usize),
	len_sq: usize,
}
impl Ord for Edge {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		// for ease of use in bin_heap where we want shortest:
		// reverse ordering directly.
		self.len_sq.cmp(&other.len_sq).reverse()
	}
}

impl PartialOrd for Edge {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

pub fn solve(input: &[u8], first_n: usize) -> Solution {
	let input = trim_end_newline(input);

	let nodes: Vec<_> = input
		.split(|&c| c == b'\n' || c == b',')
		.map(|chunk| atoi::parse_pos::<usize>(chunk).expect("valid uint"))
		.tuples()
		.map(|(x, y, z)| Node(x, y, z))
		.collect();

	// 10^6 / 2 edges :(
	// this is rather brute force, would need to look into algorithms for
	// caculating the _euclidean minimum spanning tree_ faster
	// Also I have not tested if the MinHeap is actually faster than a full sort
	let mut edges: BinaryHeap<Edge> = nodes
		.iter()
		.enumerate()
		.tuple_combinations()
		.map(|((i, Node(x1, y1, z1)), (j, Node(x2, y2, z2)))| {
			// squared length
			let len_sq =
				x1.abs_diff(*x2).pow(2) + y1.abs_diff(*y2).pow(2) + z1.abs_diff(*z2).pow(2);
			Edge {
				indices: (i, j),
				len_sq,
			}
		})
		.collect();

	let mut cycles: Vec<usize> = vec![];
	let mut cycle_membership: Vec<Option<usize>> = vec![None; nodes.len()];
	let mut big_3_cycles_after_n: usize = 0;

	let full_cycle_len = nodes.len();

	let mut loop_count = 0;
	let (_iterations, x_coords) = loop {
		loop_count += 1;
		let Edge {
			indices: (a, b), ..
		} = edges
			.pop()
			.expect("must have at least enough edges to build spanning tree");
		match (cycle_membership[a], cycle_membership[b]) {
			(None, None) => {
				let new_cycle = cycles.len();
				cycles.push(2); // new length of cycle
				cycle_membership[a] = Some(new_cycle);
				cycle_membership[b] = Some(new_cycle);
				// println!("Starting new cycle {new_cycle} for boxes [{a}, {b}]",);
			}
			(Some(a_mem), None) => {
				cycles[a_mem] += 1;
				if cycles[a_mem] == full_cycle_len {
					break (loop_count, nodes[a].0 * nodes[b].0);
				}
				cycle_membership[b] = Some(a_mem);
				// println!("Adding box {b} to cycle {a_mem}");
			}
			(None, Some(b_mem)) => {
				cycles[b_mem] += 1;
				if cycles[b_mem] == full_cycle_len {
					break (loop_count, nodes[a].0 * nodes[b].0);
				}
				cycle_membership[a] = Some(b_mem);
				// println!("Adding box {a} to cycle {b_mem}");
			}
			(Some(a_mem), Some(b_mem)) => {
				if a_mem == b_mem {
					// Already in same cycle
					// println!("same cycle");
					continue;
				}
				let (smaller, bigger) = if cycles[a_mem] < cycles[b_mem] {
					(a_mem, b_mem)
				} else {
					(b_mem, a_mem)
				};
				cycles[bigger] += cycles[smaller];
				if cycles[bigger] == full_cycle_len {
					break (loop_count, nodes[a].0 * nodes[b].0);
				}
				cycles[smaller] = 0; // tombstone value, we don't want index shifts
				for membership in cycle_membership
					.iter_mut()
					.filter(|c| c.is_some_and(|mem| mem == smaller))
				{
					membership.replace(bigger);
				}
				// println!("Merging cycle {bigger} of box {a} with cycle {smaller} of box {b}");
			}
		}
		if loop_count == first_n {
			// we do not want to disturb the original cycles vec because we're not at our goal yet
			let mut cycle_clone = cycles.clone();
			cycle_clone.sort_unstable();
			// also correct result if less than 3 big cycles, as all "default" cycles are size 1
			big_3_cycles_after_n = cycle_clone.iter().rev().take(3).product();
		}
	};

	#[cfg(test)]
	println!(
		"found minimum spanning tree in {_iterations} of {} iterations",
		edges.len()
	);

	Solution(big_3_cycles_after_n, x_coords)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		10,
		Solution(40,25272),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		1000,
		Solution(98696,2245203960),
	)]
	fn solution(#[case] input: &[u8], #[case] first_n: usize, #[case] expected: Solution) {
		assert_eq!(solve(input, first_n), expected);
	}
}
