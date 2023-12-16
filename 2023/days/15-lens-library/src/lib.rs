#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

fn hash_once(a: u8, b: u8) -> u8 {
	a.wrapping_add(b).wrapping_mul(17)
}

fn hash(bytes: &[u8], start: u8) -> u8 {
	let mut sum = start;
	for b in bytes {
		sum = hash_once(*b, sum);
	}
	sum
}

type LabelId = u64;

fn bytes_to_id(bytes: &[u8]) -> LabelId {
	let mut sum = 0_u64;
	for b in bytes {
		sum = sum << 8 | *b as u64;
	}
	sum
}

const EMPTY: Vec<(LabelId, u8)> = vec![];

pub fn solve(input: &[u8]) -> Solution {
	let mut hash_map: [Vec<(LabelId, u8)>; 256] = [EMPTY; 256];

	let mut hash_sum = 0;

	for part in input[..input.len() - 1].split(|b| *b == b',') {
		let len = part.len();
		let last = part[len - 1];
		let op_minus = last == b'-';
		let label_end = if op_minus { len - 1 } else { len - 2 };
		let label = &part[..label_end];
		let hash_sum_label = hash(label, 0);
		let hash_box = &mut hash_map[hash_sum_label as usize];
		let label_id = bytes_to_id(label);

		if op_minus {
			hash_sum += hash_once(hash_sum_label, b'-') as usize;
			hash_box.retain(|(id, _)| *id != label_id)
		} else {
			hash_sum += hash_once(hash_once(hash_sum_label, b'='), last) as usize;
			let focal_length = last - b'0';
			if let Some(pos) = hash_box.iter().position(|(id, _)| *id == label_id) {
				hash_box[pos].1 = focal_length;
			} else {
				hash_box.push((label_id, focal_length));
			}
		}
	}

	let focusing_power = hash_map
		.into_iter()
		.enumerate()
		.flat_map(|(box_index, hash_box)| {
			hash_box
				.into_iter()
				.enumerate()
				.map(move |(slot_index, (_, focal_length))| {
					(box_index + 1) * (slot_index + 1) * focal_length as usize
				})
		})
		.sum();

	Solution(hash_sum, focusing_power)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(1320,145),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(516657,210906),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
