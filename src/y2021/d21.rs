use std::{
	ops::{AddAssign, Mul},
	str::FromStr,
};

use anyhow::{anyhow, Error};
use hashbrown::HashMap;

pub struct Solution {
	with_deterministic_die: usize,
	with_dirac_die: u64,
}

struct DeterministicPlayer {
	field: u8, // 0-based (Fields 1..10)
	score: u16,
}

impl DeterministicPlayer {
	fn move_by(&mut self, amt: u16) {
		self.field = ((self.field as u16 + amt) % 10) as u8;
		self.score += self.field as u16 + 1;
	}
}

impl FromStr for DeterministicPlayer {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (_, start_field) = s.rsplit_once(' ').ok_or(anyhow!("Bad input"))?;
		let start_field = start_field.parse::<u8>()?;

		Ok(DeterministicPlayer {
			field: start_field - 1, // "field" is 0-based
			score: 0,
		})
	}
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct DiracPlayer {
	field: u8,
	score: u8,
}

impl DiracPlayer {
	fn if_moved_by(&self, amt: u8) -> DiracPlayer {
		let field = (self.field + amt) % 10;
		let score = self.score + field + 1;
		DiracPlayer { field, score }
	}
}

// P1 field, P1 score, P2 field, P2 score
type DiracMemo = HashMap<(DiracPlayer, DiracPlayer), UniverseCount>;

#[derive(Default, Clone, Debug)]
struct UniverseCount(u64, u64);

impl UniverseCount {
	fn swapped(&self) -> UniverseCount {
		UniverseCount(self.1, self.0)
	}
}

impl AddAssign for UniverseCount {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl Mul<u64> for UniverseCount {
	type Output = Self;

	fn mul(self, rhs: u64) -> Self::Output {
		UniverseCount(self.0 * rhs, self.1 * rhs)
	}
}
// 🎲🎲🎲
// 1 1 ..  3 4 5
// 1 2 ..  4 5 6
// 1 3 ..  5 6 7
// 2 ..    4 5 6 5 6 7 6 7 8
// 3 ..    5 6 7 6 7 8 7 8 9

const DIRAC_DICE_POSSIBILITIES: [(u8, u64); 7] = [
	// roll result (u8), how often (u64 for convenience)
	(3, 1),
	(4, 3),
	(5, 6),
	(6, 7),
	(7, 6),
	(8, 3),
	(9, 1),
];

const DIRAC_GOAL: u8 = 21;

fn dirac_round(memo: &mut DiracMemo, p1: DiracPlayer, p2: DiracPlayer) -> UniverseCount {
	if let Some(memoized) = memo.get(&(p1.to_owned(), p2.to_owned())) {
		return memoized.to_owned();
	}

	let mut round_count = UniverseCount::default();

	for (p1_roll, p1_times) in DIRAC_DICE_POSSIBILITIES {
		let next_p1 = p1.if_moved_by(p1_roll);

		if next_p1.score >= DIRAC_GOAL {
			round_count += UniverseCount(p1_times, 0);
			continue;
		}

		for (p2_roll, p2_times) in DIRAC_DICE_POSSIBILITIES {
			let next_p2 = p2.if_moved_by(p2_roll);

			let combined_times = p2_times * p1_times;

			round_count += if next_p2.score >= DIRAC_GOAL {
				UniverseCount(0, combined_times)
			} else {
				dirac_round(memo, next_p1.to_owned(), next_p2) * combined_times
			}
		}
	}

	// At first I felt clever and memoized this and the reverse
	// But it didn't give the right answer because p1 and p2 distinction matters
	memo.insert((p1, p2), round_count.clone());
	round_count
}

pub fn solve(input: &str) -> Solution {
	let mut die_rolls = 0;
	let mut deterministic_die = (1..=100_u16).into_iter().cycle().map(|roll| {
		die_rolls += 1;
		roll
	});

	let mut players = input.lines().map(DeterministicPlayer::from_str);
	let mut p1 = players.next().unwrap().unwrap();
	let mut p2 = players.next().unwrap().unwrap();
	assert!(players.next().is_none());

	let dirac_p1 = DiracPlayer {
		score: p1.score as u8,
		field: p1.field,
	};
	let dirac_p2 = DiracPlayer {
		score: p2.score as u8,
		field: p2.field,
	};

	let loser_score;

	// part 1
	loop {
		p1.move_by(deterministic_die.by_ref().take(3).sum());
		if p1.score >= 1000 {
			loser_score = p2.score;
			break;
		}
		p2.move_by(deterministic_die.by_ref().take(3).sum());
		if p2.score >= 1000 {
			loser_score = p1.score;
			break;
		}
	}

	let with_deterministic_die = loser_score as usize * die_rolls;

	let mut memo: DiracMemo = HashMap::new();

	let dirac_universes = dirac_round(&mut memo, dirac_p1, dirac_p2);

	let with_dirac_die = (dirac_universes.0).max(dirac_universes.1);

	Solution {
		with_deterministic_die,
		with_dirac_die,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let input = fs::read_to_string("assets/2021/input_21_sample.txt").unwrap();

		assert_eq!(solve(&input).with_deterministic_die, 739785);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_21.txt").unwrap();

		assert_eq!(solve(&input).with_deterministic_die, 1073709);
	}

	#[test]
	fn part_2_example_cases() {
		let input = fs::read_to_string("assets/2021/input_21_sample.txt").unwrap();

		assert_eq!(solve(&input).with_dirac_die, 444356092776315);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_21.txt").unwrap();

		assert_eq!(solve(&input).with_dirac_die, 148747830493442);
	}
}
