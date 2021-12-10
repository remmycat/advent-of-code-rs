use anyhow::{bail, Error};
// As much as I like to try to implement things on my own, trying to create my
// own .permutations() while knowing itertools exist was too frustrating.
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

struct Solution {
	shortest_distance: usize,
	longest_distance: usize,
}

struct Route {
	places: [String; 2],
	dist: usize,
}

impl FromStr for Route {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<_> = s.split(' ').collect();

		match parts[..] {
			[a, "to", b, "=", dist] => Ok(Route {
				places: [a.to_string(), b.to_string()],
				dist: dist.parse().unwrap(),
			}),
			_ => bail!("Bad input"),
		}
	}
}

fn get_dist(places: Vec<&String>, routes: &[Route]) -> usize {
	places
		.iter()
		.enumerate()
		.map(|(i, place)| {
			if i == 0 {
				0
			} else {
				routes
					.iter()
					.find(|route| {
						route.places.contains(place) && route.places.contains(places[i - 1])
					})
					.unwrap()
					.dist
			}
		})
		.sum()
}

fn solve(input: &str) -> Solution {
	let routes: Vec<_> = input.lines().map(|l| Route::from_str(l).unwrap()).collect();
	let unique_places: HashSet<String> = routes
		.iter()
		.flat_map(|r| r.places.clone().into_iter())
		.collect();

	let distances: Vec<_> = unique_places
		.iter()
		.permutations(unique_places.len())
		.map(|places| get_dist(places, &routes))
		.collect();

	Solution {
		shortest_distance: *distances.iter().min().unwrap(),
		longest_distance: *distances.iter().max().unwrap(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(
			solve("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141")
				.shortest_distance,
			605
		)
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2015/input_09.txt").unwrap();

		assert_eq!(solve(input.trim()).shortest_distance, 117);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(
			solve("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141")
				.longest_distance,
			982
		)
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2015/input_09.txt").unwrap();

		assert_eq!(solve(input.trim()).longest_distance, 909);
	}
}
