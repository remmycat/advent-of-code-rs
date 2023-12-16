#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

struct Num {
	value: usize,
	span: (isize, isize),
}

struct ConnectedGear {
	position: usize,
	connected_value: usize,
}

struct Gear {
	position: usize,
	connected_amt: usize,
	gear_ratio: usize,
}

impl Num {
	fn is_symbol_adjacent(
		&self,
		input: &[u8],
		in_width: isize,
		// this is to prevent constant frees and reallocation
		positions: &mut Vec<isize>,
		gears: &mut Vec<ConnectedGear>,
	) -> bool {
		let Num { span, value } = self;

		positions.clear();
		// left of number
		positions.push(span.0 - 1);
		// right of number
		positions.push(span.1 + 1);
		// above number
		positions.extend((span.0 - in_width - 1)..=(span.1 - in_width + 1));
		// below number
		positions.extend((span.0 + in_width - 1)..=(span.1 + in_width + 1));

		let mut is_symbol_adjacent = false;

		let len = input.len();

		for position in positions.iter().cloned() {
			let position = if position >= 0 {
				position as usize
			} else {
				continue;
			};
			if position >= len {
				continue;
			}

			let b = input[position];

			if b == b'*' {
				gears.push(ConnectedGear {
					position,
					connected_value: *value,
				});

				is_symbol_adjacent = true;
			} else if !is_symbol_adjacent {
				is_symbol_adjacent = !b.is_ascii_digit() && b != b'.' && b != b'\n';
			}
		}

		is_symbol_adjacent
	}
}

pub fn solve(input: &[u8]) -> Solution {
	let in_width = (input
		.iter()
		.position(|b| *b == b'\n')
		.expect("input must have newline")
		+ 1) as isize;

	let mut symbol_adjacent_sum = 0;
	let mut gear_connections: Vec<ConnectedGear> = vec![];
	let mut positions: Vec<isize> = vec![];
	let mut current_num: Option<Num> = None;

	for (position, b) in input.iter().enumerate() {
		match (&mut current_num, b) {
			(None, b) if b.is_ascii_digit() => {
				let position = position as isize;
				current_num = Some(Num {
					value: ((b - b'0') as usize),
					span: (position, position),
				});
			}
			(Some(current), b) if b.is_ascii_digit() => {
				current.span.1 += 1;
				current.value = current.value * 10 + ((b - b'0') as usize);
			}
			(None, _) => (),
			(Some(_), _) => {
				let num = current_num.unwrap();
				current_num = None;
				if num.is_symbol_adjacent(input, in_width, &mut positions, &mut gear_connections) {
					symbol_adjacent_sum += num.value;
				}
			}
		}
	}

	gear_connections.sort_unstable_by_key(|gear| gear.position);

	let gears = gear_connections
		.into_iter()
		.fold(vec![], |mut gears, connection| -> Vec<Gear> {
			if let Some(last_same) = gears
				.last_mut()
				.filter(|gear| gear.position == connection.position)
			{
				last_same.connected_amt += 1;
				if last_same.connected_amt == 2 {
					last_same.gear_ratio *= connection.connected_value
				}
			} else {
				gears.push(Gear {
					position: connection.position,
					connected_amt: 1,
					gear_ratio: connection.connected_value,
				})
			}
			gears
		});

	let gear_ratio_sum = gears
		.into_iter()
		.filter(|gear| gear.connected_amt == 2)
		.map(|gear| gear.gear_ratio)
		.sum();

	Solution(symbol_adjacent_sum, gear_ratio_sum)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(4361,467835),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(537832,81939900),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
