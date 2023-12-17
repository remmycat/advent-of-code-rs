use aoc_utils::direction::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

mod sym {
	pub const START: u8 = b'S';
	pub const NEWLINE: u8 = b'\n';
	// pub const GROUND: u8 = b'.';
	pub const VERT: u8 = b'|';
	pub const HOR: u8 = b'-';
	pub const TOP_RIGHT: u8 = b'L';
	pub const TOP_LEFT: u8 = b'J';
	pub const BOT_RIGHT: u8 = b'F';
	pub const BOT_LEFT: u8 = b'7';
}

const fn follow_facing_pipe(dir: &Direction, maybe_pipe: u8) -> Option<Direction> {
	match (dir, maybe_pipe) {
		(North, sym::VERT) | (East, sym::TOP_LEFT) | (West, sym::TOP_RIGHT) => Some(North),
		(North, sym::BOT_RIGHT) | (East, sym::HOR) | (South, sym::TOP_RIGHT) => Some(East),
		(South, sym::VERT) | (East, sym::BOT_LEFT) | (West, sym::BOT_RIGHT) => Some(South),
		(North, sym::BOT_LEFT) | (West, sym::HOR) | (South, sym::TOP_LEFT) => Some(West),
		_ => None, // this also covers ground and newline
	}
}

pub mod raycast;
pub mod shoelace_picks;

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example_1_clean(
		include_bytes!("../inputs/example_1_clean.txt"),
		Solution(4,1),
	)]
	#[case::example_1(
		include_bytes!("../inputs/example_1.txt"),
		Solution(4,1),
	)]
	#[case::example_2_clean(
		include_bytes!("../inputs/example_2_clean.txt"),
		Solution(8,1),
	)]
	#[case::example_2(
		include_bytes!("../inputs/example_2.txt"),
		Solution(8,1),
	)]
	#[case::example_3(
		include_bytes!("../inputs/example_3.txt"),
		Solution(23,4),
	)]
	#[case::example_3_squeeze(
		include_bytes!("../inputs/example_3_squeeze.txt"),
		Solution(22,4),
	)]
	#[case::example_4(
		include_bytes!("../inputs/example_4.txt"),
		Solution(70, 8),
	)]
	#[case::example_5(
		include_bytes!("../inputs/example_5.txt"),
		Solution(80, 10),
	)]
	#[case::example_6(
		include_bytes!("../inputs/example_6.txt"),
		Solution(22, 43),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(6701,303),
	)]
	fn solution(
		#[case] input: &[u8],
		#[case] expected: Solution,
		#[values(shoelace_picks::solve, raycast::solve)] solver: impl Fn(&[u8]) -> Solution,
	) {
		assert_eq!(solver(input), expected);
	}
}
