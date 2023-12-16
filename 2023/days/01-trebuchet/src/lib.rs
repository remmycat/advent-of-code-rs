#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

const LINE_SPLIT: u8 = b'\n';

fn check_spelled_digit_l(tri: &[u8], quad: Option<&u8>, quint: Option<&u8>) -> Option<usize> {
	match (tri, quad, quint) {
		(b"one", _, _) => Some(1),
		(b"two", _, _) => Some(2),
		(b"six", _, _) => Some(6),
		(b"fou", Some(b'r'), _) => Some(4),
		(b"fiv", Some(b'e'), _) => Some(5),
		(b"nin", Some(b'e'), _) => Some(9),
		(b"thr", Some(b'e'), Some(b'e')) => Some(3),
		(b"sev", Some(b'e'), Some(b'n')) => Some(7),
		(b"eig", Some(b'h'), Some(b't')) => Some(8),
		_ => None,
	}
}

fn check_spelled_digit(tri: &[u8], quad: Option<&[u8]>, quint: Option<&[u8]>) -> Option<usize> {
	match tri {
		b"one" => Some(1),
		b"two" => Some(2),
		b"six" => Some(6),
		_ => None,
	}
	.or_else(|| {
		quad.and_then(|quad| match quad {
			b"four" => Some(4),
			b"five" => Some(5),
			b"nine" => Some(9),
			_ => None,
		})
	})
	.or_else(|| {
		quint.and_then(|quint| match quint {
			b"three" => Some(3),
			b"seven" => Some(7),
			b"eight" => Some(8),
			_ => None,
		})
	})
}

fn find_first_digits(line: &[u8], rev: bool) -> (usize, usize) {
	let mut first_digit_or_spelled: Option<usize> = None;
	let mut first_digit: Option<usize> = None;

	if !rev {
		let len = line.len();
		for (index, b) in line.iter().enumerate() {
			if b.is_ascii_digit() {
				let digit = (*b - b'0') as usize;
				first_digit = Some(digit);
				first_digit_or_spelled = first_digit_or_spelled.or(first_digit);
				break;
			}

			if first_digit_or_spelled.is_some() {
				continue;
			}

			let bleft = len - index;

			let spelled = if bleft >= 3 {
				check_spelled_digit_l(
					&line[index..=(index + 2)],
					line.get(index + 3),
					line.get(index + 4),
				)
			} else {
				None
			};

			if let Some(spelled) = spelled {
				first_digit_or_spelled.replace(spelled);
				continue;
			}
		}
	} else {
		for (index, b) in line.iter().enumerate().rev() {
			if b.is_ascii_digit() {
				let digit = (*b - b'0') as usize;
				first_digit = Some(digit);
				first_digit_or_spelled = first_digit_or_spelled.or(first_digit);
				break;
			}

			if first_digit_or_spelled.is_some() {
				continue;
			}

			let spelled = if index >= 4 {
				check_spelled_digit(
					&line[(index - 2)..=index],
					Some(&line[(index - 3)..=index]),
					Some(&line[(index - 4)..=index]),
				)
			} else if index >= 3 {
				check_spelled_digit(
					&line[(index - 2)..=index],
					Some(&line[(index - 3)..=index]),
					None,
				)
			} else if index >= 2 {
				check_spelled_digit(&line[(index - 2)..=index], None, None)
			} else {
				None
			};

			if let Some(spelled) = spelled {
				first_digit_or_spelled.replace(spelled);
				continue;
			}
		}
	};

	(
		first_digit.expect("first digit should be in every input line"),
		first_digit_or_spelled
			.expect("first digit or first spelled digit should be in every input"),
	)
}

fn get_spelled_calibration_value(line: &[u8]) -> (usize, usize) {
	let (tens, tens_spelled) = find_first_digits(line, false);
	let (ones, ones_spelled) = find_first_digits(line, true);

	(tens * 10 + ones, tens_spelled * 10 + ones_spelled)
}

pub fn solve(input: &[u8]) -> Solution {
	let (simple_calibration_sum, spelled_calibration_sum): (usize, usize) = input
		.split(|b| *b == LINE_SPLIT)
		.filter(|l| !l.is_empty())
		.map(get_spelled_calibration_value)
		.fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

	Solution(simple_calibration_sum, spelled_calibration_sum)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(142 + 209,142 + 281),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(54632, 54019),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
