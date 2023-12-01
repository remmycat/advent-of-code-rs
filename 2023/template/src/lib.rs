#[derive(Debug, PartialEq, Eq)]
pub struct Solution(u64, u64);

pub fn solve(input: &[u8]) -> Solution {
	Solution(0, 0)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(todo!(), Solution(0,0))]
	#[case(todo!(), Solution(0,0))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
