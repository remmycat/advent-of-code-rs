#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

// Some musings on how to calculate all distances between points on a single axis in O(n)
//
// . point on axis
// : 2 points in one spot on axis (i.e. 2 galaxies in the same column/row)
// 0 expanding space on axis (no galaxies in this column/row)
// - path between points
// = expanding path between points (will be multiplied by 2 / 1_000_000 in the end)
// | Separator to visualise repeating path segments
//
//
// . . 0 0 . 0 . . 0 0 0 :
//
//  counting connections, adding points one by one:
//
//  -
//
//  -|- = =
//    - = =
//
//  -|- = =|- =
//    - = =|- =
//         |- =
//
//  -|- = =|- =|-
//    - = =|- =|-
//          - =|-
//              -
//
//  -|- = =|- =|-|- = = =
//    - = =|- =|-|- = = =
//          - =|-|- = = =
//              -|- = = =
//                - = = =
//
//  -|- = =|- =|-|- = = =
//    - = =|- =|-|- = = =
//          - =|-|- = = =
//              -|- = = =
//                - = = =
//
// Calculation for paths:
//
// 6*1 + 5*(3*2) + 4*(2*3) + 3*(1*4) + 2*(4*5)
//
// 6*(1-1) + 5*((3-1)*2) + 4*(2-1 * 3) + 3*(0*4) + 2*(3*5)
//
// 1
//
// 1 + 3*2
// +
// 1 + 3*2 + 2*3
// +
// 1 + 3*2 + 2*3
// +
// 1 + 3*2 + 2*3 + 1*4
// +
//(1 + 3*2 + 2*3 + 1*4 + 4*5) * 2
//
// Formula:
// total_path_sum += dist_to_last * total_point_count
// total_sum += total_path_sum * point_amt
//
// Can be split into the distances that expand and that don't:
//
// normal_path_sum += 1 * total_point_count
// normal_sum += normal_path_sum * point_amt
//
// expansive_path_sum += (dist_to_last-1) * total_point_count
// expansive_sum += expansive_path_sum * point_amt

// evaluates the sum between all point pairs for a single axis, based on a histogram
// (since the distance calculations can be fully separated per axis)
fn evaluate_axis_distances(point_counts: Vec<u8>) -> (usize, usize) {
	let mut path_sum_normal = 0; // accumulated sum of distances between points (not counting expanse)
	let mut path_sum_expanse = 0; // accumulated sum of emptiness (that expands) between points
	let mut total_point_count = 0;
	let mut last_point = 0;
	let mut sum_normal = 0;
	let mut sum_expanse = 0;

	for (point, count) in point_counts.into_iter().enumerate() {
		if count > 0 {
			let count = count as usize;
			path_sum_normal += total_point_count;
			sum_normal += path_sum_normal * count;

			let dist = point - last_point;
			if dist > 0 {
				// expanse between points
				path_sum_expanse += (dist - 1) * total_point_count;
				sum_expanse += path_sum_expanse * count;
			}

			total_point_count += count;
			last_point = point;
		}
	}

	(sum_normal, sum_expanse)
}

pub fn solve(input: &[u8]) -> Solution {
	let width = input
		.iter()
		.position(|b| *b == b'\n')
		.expect("input must have newline");

	let line_width = width + 1;
	let height = input.len() / line_width;

	let mut x_counts = vec![0_u8; width];
	let mut y_counts = vec![0_u8; height];

	for (index, _) in input.iter().enumerate().filter(|(_, b)| **b == b'#') {
		x_counts[index % line_width] += 1;
		y_counts[index / line_width] += 1;
	}

	let (x_paths, x_expanse) = evaluate_axis_distances(x_counts);
	let (y_paths, y_expanse) = evaluate_axis_distances(y_counts);

	let points_distance_base = x_paths + y_paths;
	let expanding_distance = x_expanse + y_expanse;

	let points_distance_new_galaxies = points_distance_base + expanding_distance * 2;
	let points_distance_old_galaxies = points_distance_base + expanding_distance * 1_000_000;

	Solution(points_distance_new_galaxies, points_distance_old_galaxies)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(include_bytes!("../inputs/example.txt"), Solution(374,82000210))]
	#[case::personal(include_bytes!("../inputs/personal.txt"), Solution(9918828,692506533832))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
