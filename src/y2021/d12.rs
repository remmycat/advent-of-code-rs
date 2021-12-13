use hashbrown::{HashMap, HashSet};

pub struct Solution {
	cave_paths: usize,
	cave_paths_with_extra_peek: usize,
}

struct Cave {
	big_outs: Vec<String>,
	small_outs: Vec<String>,
	is_small: bool,
	bit_mask: u16,
}

#[allow(clippy::ptr_arg)]
fn is_big_cave(s: &String) -> bool {
	s.chars().next().unwrap().is_ascii_uppercase()
}

type Tunnel = [String; 2];
impl Cave {
	fn new(name: String, index: usize, tunnels: &[Tunnel]) -> Self {
		let (big_outs, small_outs) = tunnels
			.iter()
			.filter_map(|tunnel| match tunnel {
				[a, b] if *a == name => Some(b.to_string()),
				[a, b] if *b == name => Some(a.to_string()),
				_ => None,
			})
			.partition(is_big_cave);

		let bit_mask = 1_u16 << index;

		Cave {
			big_outs,
			small_outs,
			bit_mask,
			is_small: !is_big_cave(&name),
		}
	}
}

type SmallVisited = HashSet<String>;
type Caves = HashMap<String, Cave>;

fn walk_the_caves(
	caves: &Caves,
	current: &str,
	small_visited: u16,
	extra_peek: bool,
) -> (usize, usize) {
	if current == "end" {
		return if extra_peek { (0, 1) } else { (1, 1) };
	}
	let cave = caves.get(current).unwrap();

	let mut small_vis_next = small_visited;
	if cave.is_small {
		small_vis_next += cave.bit_mask;
	}

	let mut sum = 0;
	let mut sum_with_extra = 0;
	for out in cave.big_outs.iter() {
		let res = walk_the_caves(caves, out.as_str(), small_vis_next, extra_peek);
		sum += res.0;
		sum_with_extra += res.1;
	}
	for out in cave.small_outs.iter() {
		let is_visited = small_visited & caves.get(out).unwrap().bit_mask != small_visited;

		if !is_visited {
			let res = walk_the_caves(caves, out.as_str(), small_vis_next, extra_peek);
			sum += res.0;
			sum_with_extra += res.1;
		} else if !extra_peek && out != "start" {
			sum_with_extra += walk_the_caves(caves, out.as_str(), small_vis_next, true).1;
		}
	}

	(sum, sum_with_extra)
}

pub fn solve(input: &str) -> Solution {
	let tunnels: Vec<Tunnel> = input
		.lines()
		.map(|line| {
			let mut places = line.split('-');
			[
				places.next().unwrap().to_string(),
				places.next().unwrap().to_string(),
			]
		})
		.collect();

	let caves: HashSet<String> = tunnels.clone().into_iter().flatten().collect();

	let caves: Caves = caves
		.into_iter()
		.enumerate()
		.map(|(index, name)| (name.to_string(), Cave::new(name, index, &tunnels)))
		.collect();

	let (cave_paths, cave_paths_with_extra_peek) = walk_the_caves(&caves, "start", 0, false);

	Solution {
		cave_paths,
		cave_paths_with_extra_peek,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(
			solve("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end").cave_paths,
			10
		);
		assert_eq!(
			solve(
				"dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc"
			)
			.cave_paths,
			19
		);
		assert_eq!(
			solve(
				"fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW"
			)
			.cave_paths,
			226
		);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_12.txt").unwrap();

		assert_eq!(solve(&input).cave_paths, 3887);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(
			solve(
				"dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc"
			)
			.cave_paths_with_extra_peek,
			103
		);
		assert_eq!(
			solve(
				"fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW"
			)
			.cave_paths_with_extra_peek,
			3509
		);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_12.txt").unwrap();

		assert_eq!(solve(&input).cave_paths_with_extra_peek, 104834);
	}
}
