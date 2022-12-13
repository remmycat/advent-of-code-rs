// use hashbrown::HashMap;
use std::iter::repeat;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

// Assumptions
type Worry = usize;

const NUMBERS_START: u8 = b'0';
fn parse_uint(b: &[u8]) -> usize {
	let mut num = (b[0] - NUMBERS_START) as usize;

	for digit in &b[1..] {
		if !digit.is_ascii_digit() {
			return num;
		}
		num = num * 10 + (digit - NUMBERS_START) as usize;
	}

	num
}

#[derive(Debug, Clone)]
enum WorryOperation {
	Plus(Worry),
	Times(Worry),
	Square,
}

impl WorryOperation {
	fn apply(&self, input: Worry) -> Worry {
		match self {
			Self::Plus(other) => input + *other,
			Self::Times(other) => input * *other,
			Self::Square => input * input,
		}
	}
}

#[derive(Debug, Clone)]
struct Monkey {
	items: Vec<Worry>,
	operation: WorryOperation,
	division_test: (Worry, usize, usize),
}

impl Monkey {
	fn throw_item(&mut self, no_worries: bool) -> Option<(Worry, usize)> {
		let worry = self.items.pop()?;

		// monkey is holding item
		let worry = self.operation.apply(worry);
		let worry = if no_worries { worry / 3 } else { worry };

		let (divisor, true_idx, false_idx) = self.division_test;

		Some((
			worry,
			if worry % divisor == 0 {
				true_idx
			} else {
				false_idx
			},
		))
	}
}

const ITEMS_START: usize = "  Starting items: ".len();
const OP_START: usize = "  Operation: new = old ".len();
const TEST_DIV_START: usize = "  Test: divisible by ".len();
const TEST_TRUE_START: usize = "    If true: throw to monkey ".len();
const TEST_FALSE_START: usize = "    If false: throw to monkey ".len();

impl From<&[u8]> for Monkey {
	fn from(input: &[u8]) -> Self {
		let lines: Vec<_> = input.split(|b| *b == b'\n').collect();

		let (_, items) = lines[1].split_at(ITEMS_START);
		let items: Vec<_> = items
			.split(|b| *b == b' ')
			.map(|worry| parse_uint(worry) as Worry)
			.collect();

		let (_, op) = lines[2].split_at(OP_START);
		let operation = match op[0] {
			b'+' => WorryOperation::Plus(parse_uint(&op[2..]) as Worry),
			b'*' if op[2] == b'o' => WorryOperation::Square,
			b'*' => WorryOperation::Times(parse_uint(&op[2..]) as Worry),
			_ => unimplemented!(),
		};

		let (_, divisor) = lines[3].split_at(TEST_DIV_START);
		let divisor = parse_uint(divisor) as Worry;

		let (_, true_idx) = lines[4].split_at(TEST_TRUE_START);
		let true_idx = parse_uint(true_idx);

		let (_, false_idx) = lines[5].split_at(TEST_FALSE_START);
		let false_idx = parse_uint(false_idx);

		Self {
			items,
			operation,
			division_test: (divisor, true_idx, false_idx),
		}
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let mut monkeys: Vec<_> = input
		.split(|b| *b == b'M')
		.skip(1)
		.map(Monkey::from)
		.collect();

	let monkeys_len = monkeys.len();

	let mut counts: Vec<_> = repeat(0_usize).take(monkeys_len).collect();

	let mut worrysome_monkeys = monkeys.clone();
	let mut panicked_counts = counts.clone();

	// let mut cache: HashMap<Vec<usize>, (usize, Vec<usize>)> = HashMap::new();

	let common_divisor: Worry = monkeys.iter().map(|m| m.division_test.0).product();

	for _ in 0..20 {
		for monkey_index in 0..monkeys_len {
			while let Some((item, to_index)) = monkeys[monkey_index].throw_item(true) {
				counts[monkey_index] += 1;
				monkeys[to_index].items.push(item);
			}
		}
	}

	counts.sort_unstable();
	let monkey_business_20 = counts.iter().rev().take(2).product();

	// let mut rounds = 0_usize..10000;

	// let mut caching = true;

	// while let Some(round) = rounds.next() {
	for _ in 0_usize..10_000 {
		for monkey_index in 0..monkeys_len {
			while let Some((item, to_index)) = worrysome_monkeys[monkey_index].throw_item(false) {
				panicked_counts[monkey_index] += 1;
				worrysome_monkeys[to_index]
					.items
					.push(item % common_divisor);
			}
		}
		// if caching {
		// 	// I have not proven that this horrible (fast) hash is unique for these inputs but it seems to work 😅
		// 	let state: Vec<_> = worrysome_monkeys
		// 		.iter()
		// 		.flat_map(|m| m.items.iter().cloned())
		// 		.collect();

		// 	if let Some((round_then, inspections_then)) = cache.get(&state) {
		// 		// println!("CACHE HIT AT {round} from {round_then:?}");
		// 		let round_diff = round - *round_then;
		// 		let shortcuts = rounds.len() / round_diff;
		// 		if shortcuts > 0 {
		// 			// Let's take a shortcut
		// 			for (monkey_index, old_inspection) in inspections_then.iter().enumerate() {
		// 				let current = worrysome_inspections[monkey_index];
		// 				let diff = current - *old_inspection;
		// 				worrysome_inspections[monkey_index] += diff * shortcuts;
		// 			}

		// 			// this is apparently the best way to skip elements on a mutable iterator? 🤷🏻
		// 			rounds.nth(round_diff * shortcuts - 1);
		// 		}
		// 		caching = false;
		// 	} else {
		// 		cache.insert(state, (round, worrysome_inspections.clone()));
		// 	}
		// }
	}
	panicked_counts.sort_unstable();

	let monkey_business_10000 = panicked_counts.iter().rev().take(2).product();

	Solution(monkey_business_20, monkey_business_10000)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case(include_bytes!("../inputs/example.txt"), Solution(10605,2713310158))]
	#[case(include_bytes!("../inputs/personal.txt"), Solution(56350,13954061248))]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
