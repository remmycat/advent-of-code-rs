#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

const NODE_LEN: usize = "AAA = (AAA, AAA)\n".len();
const NODE_L_START: usize = "AAA = (".len();
const NODE_R_START: usize = "AAA = (AAA, ".len();
const FIFTEEN_BIT_ARRAY_LEN: usize = 0b1_00000_00000_00000;
const A: usize = 0b00000;
const Z: usize = 0b11001;
const AAA: usize = 0b00000_00000_00000;
const ZZZ: usize = 0b11001_11001_11001;

pub enum Direction {
	Left,
	Right,
}

impl Direction {
	fn from_ascii(b: &u8) -> Direction {
		match b {
			b'L' => Direction::Left,
			b'R' => Direction::Right,
			_ => panic!("unexpected direction byte"),
		}
	}
}

// last letter is first to make checking last letter easier
fn u15_from_letters(bytes: &[u8]) -> (usize, bool) {
	// println!(
	// 	"converting {}",
	// 	String::from_utf8((bytes[..3]).to_vec()).unwrap()
	// );
	let (a, b, c) = (
		(bytes[0] - b'A') as usize,
		(bytes[1] - b'A') as usize,
		(bytes[2] - b'A') as usize,
	);
	((c << 10 | b << 5 | a), c == A)
}

pub fn solve(input: &[u8]) -> Solution {
	// storing 15 bits of data in the array index
	let mut nodes: [(usize, usize); FIFTEEN_BIT_ARRAY_LEN] = [(0, 0); FIFTEEN_BIT_ARRAY_LEN];

	let instructions_line = input
		.split(|b| *b == b'\n')
		.next()
		.expect("expected newline after instructions");

	let nodes_start = instructions_line.len() + 2;
	let nodes_len = input.len() - nodes_start;
	let nodes_amt = nodes_len / NODE_LEN;

	let instructions: Vec<_> = instructions_line
		.iter()
		.map(Direction::from_ascii)
		.collect();

	let mut end_nodes_a: Vec<usize> = vec![];

	for node_index in 0..nodes_amt {
		let start = nodes_start + NODE_LEN * node_index;
		let (u15_index, ends_a) = u15_from_letters(&input[start..]);

		if ends_a {
			end_nodes_a.push(u15_index);
		}

		nodes[u15_index] = (
			u15_from_letters(&input[(start + NODE_L_START)..]).0,
			u15_from_letters(&input[(start + NODE_R_START)..]).0,
		);

		// let x = (
		// 	u15_from_letters(&input[(start + NODE_L_START)..]),
		// 	u15_from_letters(&input[(start + NODE_R_START)..]),
		// );

		// let s = (
		// 	String::from_utf8((input[(start + NODE_L_START)..(start + NODE_L_START + 3)]).to_vec())
		// 		.unwrap(),
		// 	String::from_utf8((input[(start + NODE_R_START)..(start + NODE_R_START + 3)]).to_vec())
		// 		.unwrap(),
		// );

		// println!("{u15_index}: ({}={} , {}={})", s.0, x.0, s.1, x.1);

		// nodes[u15_index] = x;
	}

	let mut current = AAA;
	let mut end_cyle = 0;

	// take is safeguard against infinite loops
	for (index, instruction) in instructions.iter().cycle().enumerate().take(10000000) {
		// println!("current: {current}");
		if current == ZZZ {
			end_cyle = index;
			break;
		}
		match instruction {
			Direction::Left => current = nodes[current].0,
			Direction::Right => current = nodes[current].1,
		}
	}

	if end_cyle == 0 {
		panic!("ended early");
	}

	println!("{:?}", end_nodes_a);

	let mut reported: [u8; 6] = [0; 6];

	let mut end_cyle_ghosts = 0;
	// take is safeguard against infinite loops
	for (index, (inst_index, instruction)) in instructions
		.iter()
		.enumerate()
		.cycle()
		.enumerate()
		.take(10000000000000)
	{
		// println!("current: {current}");
		if end_nodes_a
			.iter()
			.enumerate()
			.any(|(i, n)| reported[i] < 2 && n >> 10 == Z)
		{
			println!("  Cycle {index} - Instruction {inst_index}");
			for (i, end_node) in end_nodes_a.iter().enumerate() {
				let hit = end_node >> 10 == Z;
				if hit {
					reported[i] += 1;
				}
				println!("[{i}]: {end_node} {:?}", end_node >> 10 == Z);
			}

			if end_nodes_a.iter().all(|n| n >> 10 == Z) {
				end_cyle_ghosts = index;
				break;
			}
		}

		match instruction {
			Direction::Left => {
				for node in end_nodes_a.iter_mut() {
					*node = nodes[*node].0;
				}
			}
			Direction::Right => {
				for node in end_nodes_a.iter_mut() {
					*node = nodes[*node].1;
				}
			}
		}
	}

	if end_cyle_ghosts == 0 {
		panic!("ended early");
	}

	Solution(end_cyle, end_cyle_ghosts)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example_1.txt"), Solution(2,2))]
	#[case(include_bytes!("../inputs/example_2.txt"), Solution(6,6))]
	#[case(include_bytes!("../inputs/example_3.txt"), Solution(1,6))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(13301,0))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
