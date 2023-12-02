pub struct Solution {
	small_cave_lowest_path_sum: usize,
	huge_cave_lowest_path_sum: usize,
}

// How many "crossroads" where paths split are considered per iteration
const TRY_LENGTH: usize = 2;
// How many of the paths we tried are allowed to continue for the next iteration
const KEEP_BEST: usize = 10_000;

// Max number of paths per iteration = KEEP_BEST * (3 ^ TRY_LENGTH)
// So it's much cheaper to scale up "KEEP_BEST", than "TRY_LENGTH"

#[derive(Clone)]
struct PathTry {
	total_score: usize,
	// x_dir: u128,
	// y_dir: u128,
	last_xy: usize,
	current_xy: usize,
}

fn get_chiton_board_score(path: &PathTry, end: usize, x_size: usize) -> usize {
	// score should reflect likelihood of path to reach end earliest
	// lower is better.
	// We don't just take lower scores because they might be further from the
	// end.
	let path_x = path.current_xy % x_size;
	let path_y = path.current_xy / x_size;
	let end_x = end % x_size;
	let end_y = end / x_size;
	let end_dist = end_x - path_x + end_y - path_y;
	// Distance is quadratic so paths have a higher incentive to get to the end.
	// Also it's a 2d map, so quadratic distance felt fitting. I have no idea.
	path.total_score + end_dist * end_dist
}

fn chiton_approval_board(
	mut paths: Vec<PathTry>,
	end: usize,
	x_size: usize,
	best: Option<usize>,
) -> Vec<PathTry> {
	// If we already have a best score, we can stop caring about every path that has a higher score.
	if let Some(best_score) = best {
		paths.retain(|p| p.total_score < best_score);
	}

	// NOTE: ideally we would first deduplicate paths that are at the same
	// position but different scores, and then decide which to keep.
	// However, because we have a big amount of paths, we deduplicate afterwards
	// and live with the possibility to keep less than KEEP_BEST paths.
	paths.sort_unstable_by_key(|path| get_chiton_board_score(path, end, x_size));
	paths.truncate(KEEP_BEST);

	// let's dedup!
	// Note that the paths are already sorted (unstably) by score.
	// We now also sort stably by current_xy.
	// So now if multiple paths have the same current_xy, they are next to each
	// other and the first of them is the one with the best score.
	paths.sort_by_key(|path| path.current_xy);
	// Then we just keep the first of every duplicate.
	paths.dedup_by_key(|path| path.current_xy);

	paths
}

fn get_chiton_approved_path(cave_map: Vec<u8>, x_size: usize, y_size: usize, end: usize) -> usize {
	// let's start in the upper left corner
	let mut paths = vec![PathTry {
		total_score: 0,
		last_xy: 0,
		current_xy: 0,
	}];

	let mut finished_paths: Vec<PathTry> = Vec::new();

	loop {
		if paths.is_empty() {
			break;
		}
		for _try in 0..=TRY_LENGTH {
			let mut new_paths: Vec<PathTry> = Vec::new();
			for path in paths.into_iter() {
				let xy = path.current_xy;
				if xy == end {
					finished_paths.push(path);
					continue;
				}
				let x = xy % x_size;
				let y = xy / x_size;
				let up = if y > 0 { Some(xy - x_size) } else { None };
				let left = if x > 0 { Some(xy - 1) } else { None };
				let bot = if y < y_size - 1 {
					Some(xy + x_size)
				} else {
					None
				};
				let right = if x < x_size - 1 { Some(xy + 1) } else { None };
				for next in [up, left, bot, right].into_iter().flatten() {
					if next != path.last_xy {
						new_paths.push(PathTry {
							last_xy: xy,
							current_xy: next,
							total_score: path.total_score + cave_map[next] as usize,
						})
					}
				}
			}
			paths = new_paths;
		}
		finished_paths.sort_by_key(|path| path.total_score);
		paths = chiton_approval_board(
			paths,
			end,
			x_size,
			finished_paths.get(0).map(|p| p.total_score),
		);
	}

	finished_paths.sort_by_key(|path| path.total_score);
	let winner = finished_paths.get(0).unwrap();

	winner.total_score
}

pub fn solve(input: &str) -> Solution {
	let mut x_size: Option<usize> = None;
	let mut y_size = 0;
	let cave_map: Vec<_> = input
		.lines()
		.flat_map(|line| {
			if x_size.is_none() {
				x_size.replace(line.len());
			}
			y_size += 1;
			line.chars().map(|c| c.to_digit(10).unwrap() as u8)
		})
		.collect();

	let x_size = x_size.unwrap();
	let end = y_size * x_size - 1;

	let huge_x_size = x_size * 5;
	let huge_y_size = y_size * 5;
	let huge_end = huge_x_size * huge_y_size - 1;

	let huge_cave_map: Vec<u8> = (0..=huge_end)
		.map(|xy| {
			let x = xy % huge_x_size;
			let small_x = x % x_size;
			let y = xy / huge_x_size;
			let small_y = y % y_size;
			let small_xy = small_x + small_y * x_size;
			let tile_bonus = x / x_size + y / y_size;
			(cave_map[small_xy] - 1 + tile_bonus as u8) % 9 + 1
		})
		.collect();

	let small_cave_lowest_path_sum = get_chiton_approved_path(cave_map, x_size, y_size, end);
	let huge_cave_lowest_path_sum =
		get_chiton_approved_path(huge_cave_map, huge_x_size, huge_y_size, huge_end);

	Solution {
		small_cave_lowest_path_sum,
		huge_cave_lowest_path_sum,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let input = fs::read_to_string("assets/15_sample.txt").unwrap();

		assert_eq!(solve(&input).small_cave_lowest_path_sum, 40);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/15.txt").unwrap();

		assert_eq!(solve(&input).small_cave_lowest_path_sum, 363);
	}

	#[test]
	fn part_2_example_cases() {
		let input = fs::read_to_string("assets/15_sample.txt").unwrap();

		assert_eq!(solve(&input).huge_cave_lowest_path_sum, 315);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/15.txt").unwrap();

		assert_eq!(solve(&input).huge_cave_lowest_path_sum, 2835);
	}
}
