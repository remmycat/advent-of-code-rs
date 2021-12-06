struct Solution {
	nice_amount: usize,
	new_rulez_nice_amount: usize,
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const NAUGHTY_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_nice(word: &&str) -> bool {
	let chars: Vec<char> = word.chars().collect();

	let has_3_vowels = chars.iter().filter(|char| VOWELS.contains(char)).count() >= 3;

	let has_pair = chars
		.iter()
		.enumerate()
		.any(|(i, char)| i != 0 && chars[i - 1] == *char);

	let has_no_naughty_strings = !NAUGHTY_STRINGS.iter().any(|naughty| word.contains(naughty));

	has_3_vowels && has_pair && has_no_naughty_strings
}

fn is_nice_new_rulez(word: &&str) -> bool {
	let chars: Vec<char> = word.chars().collect();

	let has_double_pair = chars.iter().enumerate().any(|(i, char)| {
		if i == 0 {
			return false;
		};
		let double_pat = &format!("{}{}", chars[i - 1], *char);
		word.matches(double_pat).count() >= 2
	});

	let has_skip_pair = chars
		.iter()
		.enumerate()
		.any(|(i, char)| i > 1 && chars[i - 2] == *char);

	has_double_pair && has_skip_pair
}

fn solve(input: &str) -> Solution {
	let nice_amount = input.lines().filter(is_nice).count();

	let new_rulez_nice_amount = input.lines().filter(is_nice_new_rulez).count();

	Solution {
		nice_amount,
		new_rulez_nice_amount,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve("ugknbfddgicrmopn").nice_amount, 1);
		assert_eq!(solve("aaa").nice_amount, 1);
		assert_eq!(solve("jchzalrnumimnmhp").nice_amount, 0);
		assert_eq!(solve("haegwjzuvuyypxyu").nice_amount, 0);
		assert_eq!(solve("dvszwmarrgswjxmb").nice_amount, 0);

		assert_eq!(
			solve("ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb")
				.nice_amount,
			2
		);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2015/input_05.txt").unwrap();

		assert_eq!(solve(input.trim()).nice_amount, 258);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve("qjhvhtzxzqqjkmpb").new_rulez_nice_amount, 1);
		assert_eq!(solve("xxyxx").new_rulez_nice_amount, 1);
		assert_eq!(solve("uurcxstgmygtbstg").new_rulez_nice_amount, 0);
		assert_eq!(solve("ieodomkazucvgmuy").new_rulez_nice_amount, 0);

		assert_eq!(
			solve("qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy")
				.new_rulez_nice_amount,
			2
		);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2015/input_05.txt").unwrap();

		assert_eq!(solve(input.trim()).new_rulez_nice_amount, 53);
	}
}
