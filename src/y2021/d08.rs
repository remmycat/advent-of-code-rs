use hashbrown::HashSet;

pub struct Solution {
	easy_digits: u32,
	outputs_sum: u32,
}

#[derive(PartialEq)]
struct Digit {
	segments: HashSet<char>,
}

impl Digit {
	fn new(s: &str) -> Self {
		Digit {
			segments: s.chars().collect(),
		}
	}
}

fn drain_find<F, I>(v: &mut Vec<I>, find: F) -> I
where
	F: FnMut(&I) -> bool,
{
	let index = v.iter().position(find).unwrap();
	v.remove(index)
}

fn solve_digits(ten_digits: &str) -> [Digit; 10] {
	let mut digits: Vec<Digit> = ten_digits.split(' ').map(Digit::new).collect();
	assert_eq!(digits.len(), 10);

	// 1 is the only digit with 2 segments
	let one = drain_find(&mut digits, |d| d.segments.len() == 2);

	// 4 is the only digit with 4 segments
	let four = drain_find(&mut digits, |d| d.segments.len() == 4);

	// 7 is the only digit with 3 segments
	let seven = drain_find(&mut digits, |d| d.segments.len() == 3);

	// 8 is the only digit with 8 segments
	let eight = drain_find(&mut digits, |d| d.segments.len() == 7);

	// 3 is the only 5-segment digit (2,3,5) that has all of 1's segments
	let three = drain_find(&mut digits, |d| {
		d.segments.len() == 5 && d.segments.is_superset(&one.segments)
	});

	// 6 is the only 6-segment digit (6,9,0) that does not have all of 1's segments
	let six = drain_find(&mut digits, |d| {
		d.segments.len() == 6 && !d.segments.is_superset(&one.segments)
	});

	// the segment in the upper right is left when we take all segments in 8 that are not in 3
	let seg_upper_right = eight.segments.difference(&six.segments).next().unwrap();

	// 2 is the only of the remaining 5-segment digits (2,3) that has the upper right segment
	let two = drain_find(&mut digits, |d| {
		d.segments.len() == 5 && d.segments.contains(seg_upper_right)
	});

	// 5 is the last remaining 5-segment digit
	let five = drain_find(&mut digits, |d| d.segments.len() == 5);

	// the segment in the lower left is remaining when we take all segments in 2 that are not in 3
	let seg_lower_left = two.segments.difference(&three.segments).next().unwrap();

	// 9 is the only of the remaining digits (9,0), that does not have the lower left segment
	let nine = drain_find(&mut digits, |d| !d.segments.contains(seg_lower_left));

	// 0 is the last one left
	let zero = digits.remove(0);

	[zero, one, two, three, four, five, six, seven, eight, nine]
}

pub fn solve(input: &str) -> Solution {
	let (easy_digits, outputs_sum) = input
		.lines()
		.map(|line| {
			let sides: Vec<_> = line.split(" | ").collect();
			let digits = solve_digits(sides[0]);

			let out_digits: Vec<_> = sides[1].split(' ').map(Digit::new).collect();
			let out_len = out_digits.len();

			out_digits
				.into_iter()
				.enumerate()
				.fold((0, 0), |(easy, out), (i, d)| {
					let (dig, _) = digits.iter().enumerate().find(|(_, d2)| d.eq(d2)).unwrap();
					let is_simp = dig == 1 || dig == 4 || dig == 7 || dig == 8;
					let easy = easy + is_simp as u32;
					let out2 = out + (dig as u32) * (10_u32).pow((out_len - i - 1) as u32);
					(easy, out2)
				})
		})
		.fold((0, 0), |(easy, out), (one_easy, one_out)| {
			(easy + one_easy, out + one_out)
		});

	Solution {
		easy_digits,
		outputs_sum,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let input = fs::read_to_string("assets/2021/input_08_sample.txt").unwrap();

		assert_eq!(solve(input.trim()).easy_digits, 26);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_08.txt").unwrap();

		assert_eq!(solve(input.trim()).easy_digits, 473);
	}

	#[test]
	fn part_2_example_cases() {
		let input = fs::read_to_string("assets/2021/input_08_sample.txt").unwrap();

		assert_eq!(solve(input.trim()).outputs_sum, 61229);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_08.txt").unwrap();

		assert_eq!(solve(input.trim()).outputs_sum, 1097568);
	}
}
