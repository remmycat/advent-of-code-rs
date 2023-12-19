#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

fn find_mirror_point(lines: &[&[u8]]) -> (usize, usize) {
	let width = lines[0].len();
	let height = lines.len();
	let dimension = width.max(height);

	let mut candidates_vert: Vec<_> = (1..width).map(|c| (c, false)).collect();
	let mut candidates_horz: Vec<_> = (1..height).map(|c| (c, false)).collect();

	for index in 0..dimension {
		let mut found_smudged_in_iteration = false;

		if index < height {
			let line = lines[index];

			candidates_vert.retain_mut(|(candidate, smudged)| {
				let max_count = (*candidate).min(line.len() - *candidate);
				let actual = line[0..*candidate]
					.iter()
					.rev()
					.zip(line[*candidate..].iter())
					.filter(|(a, b)| a == b)
					.count();

				match max_count - actual {
					0 => true,
					// if we already have a smudge we cannot have another!
					1 if !*smudged => {
						found_smudged_in_iteration = true;
						*smudged = true;
						true
					}
					_ => false,
				}
			})
		}

		if index < width {
			candidates_horz.retain_mut(|(candidate, smudged)| {
				let max_count = (*candidate).min(lines.len() - *candidate);

				let actual = lines[0..*candidate]
					.iter()
					.rev()
					.zip(lines[*candidate..].iter())
					.filter(|(a, b)| a[index] == b[index])
					.count();

				match max_count - actual {
					0 => true,
					// if we already have a smudge we cannot have another!
					1 if !*smudged => {
						found_smudged_in_iteration = true;
						*smudged = true;
						true
					}
					_ => false,
				}
			})
		}

		// While we will only have 2 candidates at some point (one for the smudged
		// and one for the unsmudged set), we might not have found the smudge yet!
		// We need to make sure we have found it, otherwise we don't know which of
		// the two candidates should be counted for which part result.
		if found_smudged_in_iteration && candidates_horz.len() + candidates_vert.len() == 2 {
			break;
		}
	}

	debug_assert_eq!(2, candidates_horz.len() + candidates_vert.len());

	// horizontal is times a 100
	candidates_horz
		.into_iter()
		.map(|(c, smudged)| {
			if smudged {
				(0_usize, c * 100)
			} else {
				(c * 100, 0)
			}
		})
		.chain(candidates_vert.into_iter().map(
			|(c, smudged)| {
				if smudged {
					(0_usize, c)
				} else {
					(c, 0)
				}
			},
		))
		.fold(
			(0_usize, 0_usize),
			|(acc_unsmudged, acc_smudged), (unsmudged, smudged)| {
				(acc_unsmudged + unsmudged, acc_smudged + smudged)
			},
		)
}

pub fn solve(input: &[u8]) -> Solution {
	let mut lines: Vec<&[u8]> = vec![];

	let mut unsmudged_sum = 0;
	let mut smudged_sum = 0;

	for line in input.split(|b| *b == b'\n') {
		if line.is_empty() {
			// end of block;
			let (unsmudged, smudged) = find_mirror_point(&lines);
			unsmudged_sum += unsmudged;
			smudged_sum += smudged;

			lines.clear();
			continue;
		}

		lines.push(line);
	}

	Solution(unsmudged_sum, smudged_sum)
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
