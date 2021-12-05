use std::str::FromStr;

use anyhow::{Error, Result};

struct Solution {
	gamma: i128,
	epsilon: i128,
	oxygen_generator_rating: i128,
	co2_scrubber_rating: i128,
}

#[derive(Debug, Clone)]
struct Diagnostic<const N: usize>([i128; N]);

impl<const N: usize> FromStr for Diagnostic<N> {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		let v: [char; N] = s.chars().collect::<Vec<char>>().as_slice().try_into()?;

		Ok(Diagnostic(
			v.map(|c| c.to_string().parse::<i128>().unwrap()),
		))
	}
}

impl<const N: usize> Diagnostic<N> {
	fn to_decimal(&self) -> i128 {
		let Diagnostic(dia_bits) = self;
		let mut sum: i128 = 0;
		for (i, bit) in dia_bits.iter().enumerate() {
			if *bit == 1 {
				sum += 2_i128.pow((N - 1 - i).try_into().unwrap())
			}
		}
		sum
	}
}

#[derive(Debug)]
struct DiagnosticCollection<const N: usize> {
	dias: Vec<Diagnostic<N>>,
	sum: [i128; N],
}

#[derive(Debug)]
struct DiagnosticEvaluation {
	oxygen_rating: i128,
	scrubber_rating: i128,
	gamma: i128,
	epsilon: i128,
}

fn evaluate_rating<P, const N: usize>(
	mut diagnostics: Vec<Diagnostic<N>>,
	predicate_get_comparison_bit: P,
) -> i128
where
	P: Fn(i128, i128) -> i128,
{
	for bit in 0..N {
		let leftover_len = diagnostics.len().try_into().unwrap();
		if leftover_len == 1 {
			break;
		}

		let bit_sum = diagnostics
			.to_owned()
			.into_iter()
			.fold(0_i128, |sum, Diagnostic(dia)| sum + dia[bit]);

		let comparison_bit = predicate_get_comparison_bit(leftover_len, bit_sum);

		diagnostics = diagnostics
			.into_iter()
			.filter(|Diagnostic(dia)| dia[bit] == comparison_bit)
			.collect();
	}
	assert_eq!(diagnostics.len(), 1);

	diagnostics.get(0).unwrap().to_decimal()
}

impl<const N: usize> DiagnosticCollection<N> {
	fn new() -> Self {
		DiagnosticCollection {
			dias: vec![],
			sum: [0; N],
		}
	}

	fn feed(&mut self, dia: Diagnostic<N>) {
		let Diagnostic(bits) = &dia;

		for (i, bit) in bits.iter().enumerate() {
			self.sum[i] += bit
		}

		self.dias.push(dia);
	}

	fn evaluate(&self) -> DiagnosticEvaluation {
		let mut gamma_diagnostic = Diagnostic([0; N]);
		let mut epsilon_diagnostic = Diagnostic([0; N]);

		for i in 0..N {
			// 1 more common bit: gamma gets a 1, otherwise epsilon gets a 1
			if self.sum[i] * 2 > self.dias.len().try_into().unwrap() {
				gamma_diagnostic.0[i] = 1
			} else {
				epsilon_diagnostic.0[i] = 1
			};
		}

		let oxygen_rating = evaluate_rating(
			self.dias.to_owned(),
			// are more than half the bits in this position 1s? if yes 1 is used for comparison.
			// if equal, we should also use 1 in place of the most common bit
			|len, bit_sum| if bit_sum * 2 >= len { 1 } else { 0 },
		);

		let scrubber_rating = evaluate_rating(
			self.dias.to_owned(),
			// are more than half the bits in this position 1s? if yes 0 is used for comparison.
			// if equal, we should also use 0 in place of the most common bit
			|len, bit_sum| if bit_sum * 2 >= len { 0 } else { 1 },
		);

		DiagnosticEvaluation {
			oxygen_rating,
			scrubber_rating,
			gamma: gamma_diagnostic.to_decimal(),
			epsilon: epsilon_diagnostic.to_decimal(),
		}
	}
}

fn solve<const N: usize>(input: &str) -> Solution {
	let mut dias = DiagnosticCollection::new();

	input
		.lines()
		.map(|l| l.parse::<Diagnostic<N>>().unwrap())
		.for_each(|m| dias.feed(m));

	let evaluation = dias.evaluate();

	Solution {
		oxygen_generator_rating: evaluation.oxygen_rating,
		co2_scrubber_rating: evaluation.scrubber_rating,
		epsilon: evaluation.epsilon,
		gamma: evaluation.gamma,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example =
			"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
		let solution = solve::<5>(example);

		assert_eq!(solution.gamma, 22);
		assert_eq!(solution.epsilon, 9);
		assert_eq!(solution.gamma * solution.epsilon, 198);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_03.txt").unwrap();

		let solution = solve::<12>(&input);

		assert_eq!(solution.gamma, 2601);
		assert_eq!(solution.epsilon, 1494);
		assert_eq!(solution.gamma * solution.epsilon, 3885894);
	}

	#[test]
	fn part_2_example_cases() {
		let example =
			"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
		let solution = solve::<5>(example);

		assert_eq!(solution.oxygen_generator_rating, 23);
		assert_eq!(solution.co2_scrubber_rating, 10);
		assert_eq!(
			solution.oxygen_generator_rating * solution.co2_scrubber_rating,
			230
		);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_03.txt").unwrap();

		let solution = solve::<12>(&input);

		assert_eq!(solution.oxygen_generator_rating, 3775);
		assert_eq!(solution.co2_scrubber_rating, 1159);
		assert_eq!(
			solution.oxygen_generator_rating * solution.co2_scrubber_rating,
			4375225
		);
	}
}
