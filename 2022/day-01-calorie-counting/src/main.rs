use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();

	let mut elves: Vec<u64> = vec![];
	let mut reading_elf = false;

	for line in stdin.lock().lines() {
		let line = line.expect("Could not read line from standard in");

		if line.is_empty() {
			reading_elf = false;
			continue;
		}

		let calories: u64 = line.parse().expect("input line not parsable as int");

		if !reading_elf {
			elves.push(calories);
			reading_elf = true;
			continue;
		}

		let elf = elves.last_mut().expect("impossible state");

		*elf += calories;
	}

	elves.sort_unstable();

	let max_calories: u64 = elves.iter().rev().take(1).sum();
	let max_3_calories: u64 = elves.iter().rev().take(3).sum();

	println!(r#"{{ "Top elf": {max_calories}, "Top 3 elves": {max_3_calories} }}"#);
}
