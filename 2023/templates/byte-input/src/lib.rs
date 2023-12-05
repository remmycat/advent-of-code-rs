#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

pub fn solve(_input: &[u8]) -> Solution {
	Solution(0, 0)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(0,0))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(0,0))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
