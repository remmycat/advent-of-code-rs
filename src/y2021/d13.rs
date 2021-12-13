use hashbrown::HashSet;

pub struct Solution {
	points_first_fold: usize,
	code_art: String,
}

#[derive(Clone)]
struct Fold {
	is_x: bool,
	at: isize,
}

type Coords = (isize, isize);

fn get_do_folds(folds: Vec<Fold>) -> impl Fn(Coords) -> Coords {
	move |mut coords: Coords| -> Coords {
		for fold in folds.iter() {
			if fold.is_x {
				if coords.0 > fold.at {
					coords.0 -= (coords.0 - fold.at) * 2
				}
			} else if coords.1 > fold.at {
				coords.1 -= (coords.1 - fold.at) * 2
			}
		}

		coords
	}
}

pub fn solve(input: &str) -> Solution {
	let mut input = input.split("\n\n");
	let coords_str = input.next().unwrap();

	let mut first_fold: Option<Fold> = None;
	let folds: Vec<_> = input
		.next()
		.unwrap()
		.lines()
		.map(|line| {
			let parts: Vec<_> = line.split(&[' ', '='][..]).collect();
			let fold = match parts[..] {
				["fold", "along", "x", x] => Fold {
					is_x: true,
					at: x.parse().unwrap(),
				},
				["fold", "along", "y", y] => Fold {
					is_x: false,
					at: y.parse().unwrap(),
				},
				_ => panic!("Bad input"),
			};
			if first_fold.is_none() {
				first_fold = Some(fold.clone());
			}
			fold
		})
		.collect();

	let first_fold = first_fold.unwrap();
	let do_first_fold = get_do_folds(vec![first_fold]);

	let after_one_fold: HashSet<Coords> = coords_str
		.lines()
		.map(|line| {
			let mut x_y = line.split(',');
			(
				x_y.next().unwrap().parse().unwrap(),
				x_y.next().unwrap().parse().unwrap(),
			)
		})
		.map(do_first_fold)
		.collect();

	let do_all_folds = get_do_folds(folds);

	let after_all_folds: HashSet<Coords> = coords_str
		.lines()
		.map(|line| {
			let mut x_y = line.split(',');
			(
				x_y.next().unwrap().parse().unwrap(),
				x_y.next().unwrap().parse().unwrap(),
			)
		})
		.map(do_all_folds)
		.collect();

	let x_max = after_all_folds.iter().map(|(x, _)| x).max().unwrap();
	let y_max = after_all_folds.iter().map(|(_, y)| y).max().unwrap();

	let mut code_art: Vec<char> = Vec::new();
	for y in 0..=*y_max {
		for x in 0..=*x_max {
			code_art.push(if after_all_folds.contains(&(x, y)) {
				'█'
			} else {
				'░'
			});
		}
		code_art.push('\n');
	}
	let code_art = code_art.into_iter().collect();

	Solution {
		points_first_fold: after_one_fold.len(),
		code_art,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;
		assert_eq!(solve(example.trim()).points_first_fold, 17);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2021/input_13.txt").unwrap();

		assert_eq!(solve(&input).points_first_fold, 592);
	}

	#[test]
	fn part_2_example_cases() {
		let example = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

		let expected = r#"
█████
█░░░█
█░░░█
█░░░█
█████
"#;

		assert_eq!(solve(example.trim()).code_art.trim(), expected.trim());
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2021/input_13.txt").unwrap();

		let expected = r#"
░░██░░██░░░██░░░░██░████░████░█░░█░█░░█
░░░█░█░░█░█░░█░░░░█░█░░░░█░░░░█░█░░█░░█
░░░█░█░░░░█░░█░░░░█░███░░███░░██░░░█░░█
░░░█░█░██░████░░░░█░█░░░░█░░░░█░█░░█░░█
█░░█░█░░█░█░░█░█░░█░█░░░░█░░░░█░█░░█░░█
░██░░░███░█░░█░░██░░████░█░░░░█░░█░░██░
"#;

		assert_eq!(solve(&input).code_art.trim(), expected.trim());
	}
}
