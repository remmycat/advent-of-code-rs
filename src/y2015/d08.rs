struct Solution {
	code_str_overhead: usize,
	escaped_code_str_overhead: usize,
}

const MATCH_ESC_BACK: &str = r#"\\"#;
const MATCH_UNICODE: &str = r#"\x"#;
const MATCH_ESC: &str = r#"\""#;

fn solve(input: &str) -> Solution {
	let code_str_overhead = input
		.lines()
		.map(|line| {
			let mut code_str = line.to_string();
			let mut overhead = 2;

			overhead += code_str.matches(MATCH_ESC_BACK).count();
			code_str = code_str.replace(MATCH_ESC_BACK, "");

			overhead += code_str.matches(MATCH_UNICODE).count() * 3;
			code_str = code_str.replace(MATCH_UNICODE, "");

			overhead += code_str.matches(MATCH_ESC).count();

			overhead
		})
		.sum();

	let escaped_code_str_overhead = input
		.lines()
		.map(|line| {
			let mut code_str = line.to_string();
			let mut overhead = 4;

			overhead += code_str.matches(MATCH_ESC_BACK).count() * 2;
			code_str = code_str.replace(MATCH_ESC_BACK, "");

			overhead += code_str.matches(MATCH_UNICODE).count();
			code_str = code_str.replace(MATCH_UNICODE, "");

			overhead += code_str.matches(MATCH_ESC).count() * 2;

			overhead
		})
		.sum();

	Solution {
		code_str_overhead,
		escaped_code_str_overhead,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		assert_eq!(solve(r#""""#).code_str_overhead, 2);
		assert_eq!(solve(r#""abc""#).code_str_overhead, 2);
		assert_eq!(solve(r#""aaa\"aaa""#).code_str_overhead, 3);
		assert_eq!(solve(r#""\x27""#).code_str_overhead, 5);

		let combined = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#;
		assert_eq!(solve(combined.trim()).code_str_overhead, 12);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2015/input_08.txt").unwrap();

		assert_eq!(solve(input.trim()).code_str_overhead, 1333);
	}

	#[test]
	fn part_2_example_cases() {
		assert_eq!(solve(r#""""#).escaped_code_str_overhead, 4);
		assert_eq!(solve(r#""abc""#).escaped_code_str_overhead, 4);
		assert_eq!(solve(r#""aaa\"aaa""#).escaped_code_str_overhead, 6);
		assert_eq!(solve(r#""\x27""#).escaped_code_str_overhead, 5);

		let combined = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#;
		assert_eq!(solve(combined.trim()).escaped_code_str_overhead, 19);
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2015/input_08.txt").unwrap();

		assert_eq!(solve(input.trim()).escaped_code_str_overhead, 2046);
	}
}
