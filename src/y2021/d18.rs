use std::{iter::Peekable, ops::Add, str::Chars};

use itertools::Itertools;

pub struct Solution {
	sum_magnitude: usize,
	highest_sum_magnitude: usize,
}

#[derive(Clone)]
enum SnailValue {
	Num(usize),
	Pair(Box<SnailNumber>),
}

#[derive(Clone)]
struct SnailNumber(SnailValue, SnailValue);

fn snail_split(val: usize) -> SnailNumber {
	let lnew = SnailValue::Num(val / 2);
	let rnew = SnailValue::Num(val / 2 + val % 2);

	SnailNumber(lnew, rnew)
}

fn add_to_leftmost(value: &mut SnailValue, to_add: usize) {
	match value {
		SnailValue::Num(val) => *val += to_add,
		SnailValue::Pair(lpair) => {
			add_to_leftmost(&mut lpair.0, to_add);
		}
	}
}

fn add_to_rightmost(value: &mut SnailValue, to_add: usize) {
	match value {
		SnailValue::Num(val) => *val += to_add,
		SnailValue::Pair(rpair) => {
			add_to_rightmost(&mut rpair.1, to_add);
		}
	}
}

impl SnailNumber {
	fn explode(&mut self, depth: usize) -> Option<(Option<usize>, Option<usize>)> {
		use SnailValue::*;
		if depth == 5 {
			if let SnailNumber(Num(a), Num(b)) = self {
				// 💃 https://youtu.be/7r-tq-M77VY?t=13
				return Some((Some(*a), Some(*b)));
			} else {
				panic!("Pairs at level 5 should always be two numbers!")
			}
		}

		match self {
			SnailNumber(Num(_), Num(_)) => None,
			SnailNumber(left @ Pair(_), Num(right)) => {
				let lpair = if let Pair(lpair) = left {
					lpair
				} else {
					panic!("Impossible")
				};
				lpair.explode(depth + 1).map(|(lrest, rrest)| {
					if depth == 4 {
						*left = SnailValue::Num(0);
					}
					if let Some(radd) = rrest {
						*right += radd;
					};
					(lrest, None)
				})
			}
			SnailNumber(Num(left), right @ Pair(_)) => {
				let rpair = if let Pair(rpair) = right {
					rpair
				} else {
					panic!("Impossible")
				};
				rpair.explode(depth + 1).map(|(lrest, rrest)| {
					if depth == 4 {
						*right = SnailValue::Num(0);
					}
					if let Some(ladd) = lrest {
						*left += ladd;
					};
					(None, rrest)
				})
			}
			SnailNumber(left @ Pair(_), right @ Pair(_)) => {
				let lpair = if let Pair(lpair) = left {
					lpair
				} else {
					panic!("Impossible")
				};

				let rpair = if let Pair(rpair) = right {
					rpair
				} else {
					panic!("Impossible")
				};

				// A bit more verbose so the borrow checker really understands all possible paths

				if let Some((lrest, rrest)) = lpair.explode(depth + 1) {
					if depth == 4 {
						*left = SnailValue::Num(0);
					}
					if let Some(radd) = rrest {
						add_to_leftmost(&mut rpair.0, radd);
					};
					Some((lrest, None))
				} else if let Some((lrest, rrest)) = rpair.explode(depth + 1) {
					if depth == 4 {
						*right = SnailValue::Num(0);
					}
					if let Some(ladd) = lrest {
						add_to_rightmost(&mut lpair.1, ladd);
					};
					Some((None, rrest))
				} else {
					None
				}
			}
		}
	}

	fn split(&mut self) -> bool {
		let SnailNumber(left, right) = self;

		match left {
			SnailValue::Num(lnum) => {
				if *lnum > 9 {
					*left = SnailValue::Pair(Box::new(snail_split(*lnum)));
					return true;
				}
			}
			SnailValue::Pair(lpair) => {
				if lpair.split() {
					return true;
				}
			}
		}

		match right {
			SnailValue::Num(rnum) => {
				if *rnum > 9 {
					*right = SnailValue::Pair(Box::new(snail_split(*rnum)));
					return true;
				}
			}
			SnailValue::Pair(rpair) => {
				if rpair.split() {
					return true;
				}
			}
		}

		false
	}

	fn reduce(&mut self) {
		loop {
			if self.explode(1).is_some() {
				continue;
			}
			if self.split() {
				continue;
			}
			break;
		}
	}

	fn from_chars(chars: &mut Peekable<Chars>) -> Self {
		assert_eq!(chars.next().unwrap(), '[');

		let left = match *chars.peek().unwrap() {
			'[' => SnailValue::Pair(Box::new(Self::from_chars(chars))),
			_ => {
				let dig = chars.next().unwrap();
				SnailValue::Num(dig.to_digit(10).unwrap() as usize)
			}
		};

		assert_eq!(chars.next().unwrap(), ',');

		let right = match *chars.peek().unwrap() {
			'[' => SnailValue::Pair(Box::new(Self::from_chars(chars))),
			_ => {
				let dig = chars.next().unwrap();
				SnailValue::Num(dig.to_digit(10).unwrap() as usize)
			}
		};

		assert_eq!(chars.next().unwrap(), ']');

		Self(left, right)
	}

	fn magnitude(&self) -> usize {
		let SnailNumber(left, right) = self;

		let mag_left = 3 * match left {
			SnailValue::Num(lnum) => *lnum,
			SnailValue::Pair(lpair) => lpair.magnitude(),
		};

		let mag_right = 2 * match right {
			SnailValue::Num(rnum) => *rnum,
			SnailValue::Pair(rpair) => rpair.magnitude(),
		};

		mag_left + mag_right
	}
}

impl Add for SnailNumber {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let left = SnailValue::Pair(Box::new(self));
		let right = SnailValue::Pair(Box::new(other));
		let mut result = SnailNumber(left, right);
		result.reduce();

		result
	}
}

pub fn solve(input: &str) -> Solution {
	let input_numbers: Vec<_> = input
		.lines()
		.map(|line| {
			let mut chars = line.chars().peekable();
			SnailNumber::from_chars(&mut chars)
		})
		.collect();

	let sum: Option<SnailNumber> = input_numbers.iter().fold(None, |sum, next| {
		if let Some(prev) = sum {
			Some(prev + next.clone())
		} else {
			Some(next.clone())
		}
	});

	let sum = sum.unwrap();
	let sum_magnitude = sum.magnitude();

	let highest_sum_magnitude = input_numbers
		.into_iter()
		.tuple_combinations::<(_, _)>()
		.map(|(a, b)| {
			let mag1 = (a.clone() + b.clone()).magnitude();
			let mag2 = (b + a).magnitude();
			mag1.max(mag2)
		})
		.max()
		.unwrap();

	Solution {
		sum_magnitude,
		highest_sum_magnitude,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("[9,1]").sum_magnitude, 29);
		assert_eq!(solve("[[1,2],[[3,4],5]]").sum_magnitude, 143);
		assert_eq!(
			solve("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").sum_magnitude,
			1384
		);
		assert_eq!(solve("[[[[1,1],[2,2]],[3,3]],[4,4]]").sum_magnitude, 445);
		assert_eq!(solve("[[[[3,0],[5,3]],[4,4]],[5,5]]").sum_magnitude, 791);
		assert_eq!(solve("[[[[5,0],[7,4]],[5,5]],[6,6]]").sum_magnitude, 1137);
		assert_eq!(
			solve("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").sum_magnitude,
			3488
		);

		assert_eq!(solve("[1,2]\n[[3,4],5]").sum_magnitude, 143);

		let input = fs::read_to_string("assets/2021/input_18_sample.txt").unwrap();

		assert_eq!(solve(&input).sum_magnitude, 4140);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_18.txt").unwrap();

		assert_eq!(solve(&input).sum_magnitude, 4433);
	}

	#[test]
	fn part_2_example_cases() {
		let input = fs::read_to_string("assets/2021/input_18_sample.txt").unwrap();

		assert_eq!(solve(&input).highest_sum_magnitude, 3993);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_18.txt").unwrap();

		assert_eq!(solve(&input).highest_sum_magnitude, 4559);
	}
}
