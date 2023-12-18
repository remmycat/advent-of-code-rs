#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

fn find_mirror_point(lines: &[&[u8]]) -> ((usize, usize), (usize, usize)) {
	let width = lines[0].len();
	let height = lines.len();
	let dimension = width.max(height);

	let mut candidates_vert: Vec<_> = (1..width).collect();
	let mut candidates_horz: Vec<_> = (1..height).collect();

	let mut candidates_vert_smudged: Vec<_> = (1..width).map(|c| (c, false)).collect();
	let mut candidates_horz_smudged: Vec<_> = (1..height).map(|c| (c, false)).collect();

	for index in 0..dimension {
		if index < height {
			let line = lines[index];
			candidates_vert.retain(|candidate| {
				line[0..*candidate]
					.iter()
					.rev()
					.zip(line[*candidate..].iter())
					.all(|(a, b)| a == b)
			});
		}

		if index < width {
			candidates_horz.retain(|candidate| {
				lines[0..*candidate]
					.iter()
					.rev()
					.zip(lines[*candidate..].iter())
					.all(|(a, b)| a[index] == b[index])
			})
		}

		if candidates_horz.len() + candidates_vert.len() == 1 {
			break;
		}
	}

	if candidates_horz.len() + candidates_vert.len() != 1 {
		panic!("expected candidates reduced to 1")
	}

	let unsmudged = if let Some(h_candidate) = candidates_horz.first() {
		// remove unsmudged find from smudged candidates so we wont find it again
		candidates_horz_smudged.retain(|(c, _)| *c != *h_candidate);
		(0_usize, *h_candidate)
	} else {
		// remove unsmudged find from smudged candidates so we wont find it again
		let v_candidate = candidates_vert.first().unwrap();
		candidates_vert_smudged.retain(|(c, _)| *c != *v_candidate);
		(*v_candidate, 0_usize)
	};

	for index in 0..dimension {
		if index < height {
			let line = lines[index];

			candidates_vert_smudged.retain_mut(|(candidate, smudged)| {
				if *smudged {
					// already has a smudge cannot have another!
					line[0..*candidate]
						.iter()
						.rev()
						.zip(line[*candidate..].iter())
						.all(|(a, b)| a == b)
				} else {
					let max_count = (*candidate).min(line.len() - *candidate);
					let actual = line[0..*candidate]
						.iter()
						.rev()
						.zip(line[*candidate..].iter())
						.filter(|(a, b)| a == b)
						.count();

					match max_count - actual {
						0 => true,
						1 => {
							*smudged = true;
							true
						}
						_ => false,
					}
				}
			})
		}

		if index < width {
			candidates_horz_smudged.retain_mut(|(candidate, smudged)| {
				if *smudged {
					// already has a smudge cannot have another!
					lines[0..*candidate]
						.iter()
						.rev()
						.zip(lines[*candidate..].iter())
						.all(|(a, b)| a[index] == b[index])
				} else {
					let max_count = (*candidate).min(lines.len() - *candidate);

					let actual = lines[0..*candidate]
						.iter()
						.rev()
						.zip(lines[*candidate..].iter())
						.filter(|(a, b)| a[index] == b[index])
						.count();

					match max_count - actual {
						0 => true,
						1 => {
							*smudged = true;
							true
						}
						_ => false,
					}
				}
			})
		}

		if candidates_horz_smudged.len() + candidates_vert_smudged.len() == 1 {
			break;
		}
	}

	if candidates_horz_smudged.len() + candidates_vert_smudged.len() != 1 {
		panic!("expected smudged candidates reduced to 1")
	}

	let smudged = candidates_horz_smudged
		.first()
		.map(|(c, _)| (0_usize, *c))
		.or(candidates_vert_smudged.first().map(|(c, _)| (*c, 0_usize)))
		.unwrap();

	(unsmudged, smudged)
}

pub fn solve(input: &[u8]) -> Solution {
	let mut lines: Vec<&[u8]> = vec![];

	let mut vertical_sum = 0;
	let mut horizontal_sum = 0;

	let mut vertical_sum_smudged = 0;
	let mut horizontal_sum_smudged = 0;

	for line in input.split(|b| *b == b'\n') {
		if line.is_empty() {
			// end of block;
			let ((vert, horz), (vert_smudged, horz_smudged)) = find_mirror_point(&lines);
			vertical_sum += vert;
			horizontal_sum += horz;

			vertical_sum_smudged += vert_smudged;
			horizontal_sum_smudged += horz_smudged;

			lines.clear();
			continue;
		}

		lines.push(line);
	}

	let sum = horizontal_sum * 100 + vertical_sum;

	let sum_smudged = horizontal_sum_smudged * 100 + vertical_sum_smudged;

	Solution(sum, sum_smudged)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(405,400),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(28895,31603),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
