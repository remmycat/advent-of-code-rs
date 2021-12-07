use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Error, Result};

type Bits = u16;

type WireSalad = HashMap<String, WireCommand>;
type Cache = HashMap<String, Bits>;

#[derive(Debug)]
enum Input {
	SignalIn(Bits),
	WireOut(String),
}

impl Input {
	fn signal(&self, wires: &WireSalad, cache: &mut Cache) -> Bits {
		match self {
			Self::SignalIn(bits) => *bits,
			Self::WireOut(wire) => {
				if cache.contains_key(wire) {
					*cache.get(wire).unwrap()
				} else {
					match wires.get(wire) {
						None => 0,
						Some(cmd) => {
							let out = cmd.out(wires, cache);
							cache.insert(wire.to_owned(), out);
							out
						}
					}
				}
			}
		}
	}
}

impl FromStr for Input {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let first_char = s.to_owned().chars().next().unwrap();
		if first_char.is_ascii_lowercase() {
			Ok(Self::WireOut(s.to_string()))
		} else if first_char.is_ascii_digit() {
			Ok(Self::SignalIn(s.parse::<Bits>().unwrap()))
		} else {
			bail!("Bad Input")
		}
	}
}

#[derive(Debug)]

enum WireCommand {
	In(Input),
	Not(Input),
	And(Input, Input),
	Or(Input, Input),
	LShift(Input, u8),
	RShift(Input, u8),
}

impl WireCommand {
	fn out(&self, w: &WireSalad, c: &mut Cache) -> Bits {
		match self {
			Self::In(in1) => in1.signal(w, c),
			Self::Not(in1) => !in1.signal(w, c),
			Self::And(in1, in2) => in1.signal(w, c) & in2.signal(w, c),
			Self::Or(in1, in2) => in1.signal(w, c) | in2.signal(w, c),
			Self::LShift(in1, amt) => in1.signal(w, c) << amt,
			Self::RShift(in1, amt) => in1.signal(w, c) >> amt,
		}
	}
}

struct Solution {
	wire_out: Bits,
	b_reroute_wire_out: Bits,
}

fn solve(input: &str, wire: &str) -> Result<Solution> {
	let mut salad: WireSalad = HashMap::new();

	for line in input.lines() {
		use WireCommand as WC;

		let v: Vec<_> = line.split(' ').collect();

		let (name, cmd) = match v[..] {
			["NOT", in1, "->", name] => (name, WC::Not(in1.parse().unwrap())),
			[in1, "->", name] => (name, WC::In(in1.parse().unwrap())),
			[in1, "AND", in2, "->", name] => {
				(name, WC::And(in1.parse().unwrap(), in2.parse().unwrap()))
			}
			[in1, "OR", in2, "->", name] => {
				(name, WC::Or(in1.parse().unwrap(), in2.parse().unwrap()))
			}
			[in1, "LSHIFT", in2, "->", name] => {
				(name, WC::LShift(in1.parse().unwrap(), in2.parse().unwrap()))
			}
			[in1, "RSHIFT", in2, "->", name] => {
				(name, WC::RShift(in1.parse().unwrap(), in2.parse().unwrap()))
			}
			_ => bail!("Bad Input"),
		};

		salad.insert(name.to_string(), cmd);
	}

	let mut cache: Cache = HashMap::new();
	let wire_out = salad.get(wire).unwrap().out(&salad, &mut cache);

	salad.insert("b".to_string(), WireCommand::In(Input::SignalIn(wire_out)));
	cache.clear();
	let b_reroute_wire_out = salad.get(wire).unwrap().out(&salad, &mut cache);

	Ok(Solution {
		wire_out,
		b_reroute_wire_out,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn part_1_example_cases() {
		let example = r#"
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
"#;

		assert_eq!(solve(example.trim(), "d").unwrap().wire_out, 72);
		assert_eq!(solve(example.trim(), "e").unwrap().wire_out, 507);
		assert_eq!(solve(example.trim(), "f").unwrap().wire_out, 492);
		assert_eq!(solve(example.trim(), "g").unwrap().wire_out, 114);
		assert_eq!(solve(example.trim(), "h").unwrap().wire_out, 65412);
		assert_eq!(solve(example.trim(), "i").unwrap().wire_out, 65079);
		assert_eq!(solve(example.trim(), "x").unwrap().wire_out, 123);
		assert_eq!(solve(example.trim(), "y").unwrap().wire_out, 456);
	}

	#[test]
	fn part_1_solution() {
		let input = fs::read_to_string("assets/2015/input_07.txt").unwrap();

		assert_eq!(solve(input.trim(), "a").unwrap().wire_out, 16076);
	}

	#[test]
	fn part_2_example_cases() {
		// No examples here
	}

	#[test]
	fn part_2_solution() {
		let input = fs::read_to_string("assets/2015/input_07.txt").unwrap();

		assert_eq!(solve(input.trim(), "a").unwrap().b_reroute_wire_out, 2797);
	}
}
