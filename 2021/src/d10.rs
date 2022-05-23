pub struct Solution {
	corrupted_score: usize,
	unclosed_middle_score: usize,
}

enum LineScore {
	Corrupted(usize),
	Unclosed(usize),
}

fn get_error_score(line: &str) -> LineScore {
	let mut chunk_stack = Vec::new();

	for c in line.chars() {
		match c {
			'(' => chunk_stack.push(')'),
			'[' => chunk_stack.push(']'),
			'{' => chunk_stack.push('}'),
			'<' => chunk_stack.push('>'),
			closing => {
				if chunk_stack.is_empty() || closing != chunk_stack.pop().unwrap() {
					return match closing {
						')' => LineScore::Corrupted(3),
						']' => LineScore::Corrupted(57),
						'}' => LineScore::Corrupted(1197),
						'>' => LineScore::Corrupted(25137),
						_ => panic!("Bad input"),
					};
				}
			}
		};
	}

	LineScore::Unclosed(chunk_stack.iter().rfold(0, |score, c| {
		let value = match c {
			')' => 1,
			']' => 2,
			'}' => 3,
			'>' => 4,
			_ => panic!("Bad input"),
		};

		score * 5 + value
	}))
}

pub fn solve(input: &str) -> Solution {
	let scores: Vec<_> = input.lines().map(get_error_score).collect();

	let corrupted_score = scores
		.iter()
		.filter_map(|ls| match ls {
			LineScore::Corrupted(score) => Some(*score),
			_ => None,
		})
		.sum();

	let mut unclosed_scores: Vec<_> = scores
		.iter()
		.filter_map(|ls| match ls {
			LineScore::Unclosed(score) => Some(*score),
			_ => None,
		})
		.collect();

	unclosed_scores.sort_unstable();
	let unclosed_middle_score = unclosed_scores[unclosed_scores.len() / 2];

	Solution {
		corrupted_score,
		unclosed_middle_score,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;
		assert_eq!(solve(example.trim()).corrupted_score, 26397);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/10.txt").unwrap();

		assert_eq!(solve(input.trim()).corrupted_score, 271245);
	}

	#[test]
	fn part_2_example_cases() {
		let example = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;
		assert_eq!(solve(example.trim()).unclosed_middle_score, 288957);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/10.txt").unwrap();

		assert_eq!(solve(input.trim()).unclosed_middle_score, 1685293086);
	}
}
