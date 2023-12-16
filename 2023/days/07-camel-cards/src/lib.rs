use aoc_utils::ascii_int::parse_uint;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

struct Hand {
	score: u32,
	score_joker: u32,
	bet: usize,
}

impl Hand {
	fn get_card_value(card: u8) -> u8 {
		match card {
			b if b.is_ascii_digit() => card - b'1',
			// b'2' => 1, // I'm so sorry, it's faster, I swear
			// b'3' => 2,
			// ...
			b'T' => 9,
			b'J' => 10,
			b'Q' => 11,
			b'K' => 12,
			b'A' => 13,
			_ => panic!("expected card identifier"),
		}
	}

	// encode 5s,4s,3s,pairs as 5 bits (only pairs can be 2 bits)
	fn collections_score(collections: [u8; 6]) -> u8 {
		let mut score = collections[5] << 4;
		score |= collections[4] << 3;
		score |= collections[3] << 2;
		score | collections[2]
	}

	fn parse(line: &[u8]) -> Hand {
		let bet = parse_uint(&line[6..]);

		let mut card_amts = [0_u8; 14];
		let mut base_score: u32 = 0; // right 20 bits are base_score
		let mut base_score_joker: u32 = 0;

		for card in line[..5].iter() {
			let card_value = Self::get_card_value(*card);
			// base score: each card value, leftmost is best
			base_score = (base_score << 4) | card_value as u32;

			// map jokers to 0 for score with jokers -> less valuable than 2
			let card_value = if card_value == 10 { 0 } else { card_value };
			base_score_joker = (base_score_joker << 4) | card_value as u32;

			card_amts[card_value as usize] += 1;
		}

		let joker_amt = card_amts[0] as usize;
		let mut highest_non_joker_amt: usize = 0;

		let mut collections = [0_u8; 6];

		for (card_index, card_amt) in card_amts.into_iter().enumerate() {
			let card_amt = card_amt as usize;
			collections[card_amt] += 1;

			if card_index > 0 && card_amt > highest_non_joker_amt {
				highest_non_joker_amt = card_amt
			}
		}

		let pair_score = Self::collections_score(collections);

		let pair_score_joker = if joker_amt == 0 || joker_amt == 5 {
			pair_score
		} else {
			collections[highest_non_joker_amt] -= 1;
			collections[joker_amt] -= 1;
			collections[highest_non_joker_amt + joker_amt] += 1;
			Self::collections_score(collections)
		};

		Hand {
			score: base_score | (pair_score as u32) << 20,
			score_joker: base_score_joker | (pair_score_joker as u32) << 20,
			bet,
		}
	}
}

pub fn solve(input: &[u8]) -> Solution {
	// let mut card_store = CardCollection::new();
	let mut hands: Vec<_> = input
		.split(|b| *b == b'\n')
		.filter(|l| !l.is_empty())
		.map(Hand::parse)
		.collect();

	hands.sort_unstable_by_key(|hand| hand.score); // ~8 us for sorting

	let winnings = hands
		.iter()
		.enumerate()
		.map(|(index, hand)| hand.bet * (index + 1))
		.sum();

	hands.sort_unstable_by_key(|hand| hand.score_joker); // ~8 us for sorting

	let winnings_joker = hands
		.iter()
		.enumerate()
		.map(|(index, hand)| hand.bet * (index + 1))
		.sum();

	Solution(winnings, winnings_joker)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(6440,5905),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(248453531,248781813),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
