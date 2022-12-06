pub mod solve_hashset;
pub mod solve_loop;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(u64, u64);

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example_5.txt"), Solution(5,23))]
	#[case(include_bytes!("../inputs/example_6.txt"), Solution(6,23))]
	#[case(include_bytes!("../inputs/example_7.txt"), Solution(7,19))]
	#[case(include_bytes!("../inputs/example_10.txt"), Solution(10,29))]
	#[case(include_bytes!("../inputs/example_11.txt"), Solution(11,26))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(1142,2803))]
	fn solution(
		#[case] input: &[u8],
		#[case] expected: Solution,
		#[values(solve_hashset::solve_hashset, solve_loop::solve_loop)] solve: impl Fn(
			&[u8],
		) -> Solution,
	) {
		assert_eq!(solve(input), expected);
	}
}
