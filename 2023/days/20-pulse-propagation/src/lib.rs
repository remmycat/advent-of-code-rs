use std::{collections::VecDeque, fmt::Debug};

use aoc_utils::{lcm::lowest_common_multiple, trim::trim_end_newline};
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
	Hi,
	Lo,
}

impl Pulse {
	fn toggle(&mut self) {
		*self = match self {
			Hi => Lo,
			Lo => Hi,
		}
	}
}

use Pulse::*;

#[derive(Clone, PartialEq, Eq, Hash)]
struct ModuleName(Vec<u8>);

impl Debug for ModuleName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let as_str = String::from_utf8(self.0.clone()).expect("name must be valid utf8");
		write!(f, "[{as_str}]")
	}
}

#[derive(Debug, Clone)]
struct Cable {
	from_conjunction: Option<ModuleName>,
	to: ModuleName,
}

const ARROW_LEN: usize = b" ->".len();
const BROADCASTER_NAME: &[u8] = b"broadcaster";
const BROADCAST_LEN: usize = BROADCASTER_NAME.len();

#[derive(Debug)]
enum Module {
	Broadcaster {
		outs: (usize, usize),
	},
	// Needs to know identities of inputs mapped to internal state
	Conjunction {
		ins: Vec<(usize, Pulse)>,
		outs: (usize, usize),
	},
	// needs to know current state only
	FlipFlop {
		toggle: Pulse,
		outs: (usize, usize),
	},
}

#[derive(Debug, Default)]
struct Circuit {
	hi_count: usize,
	lo_count: usize,
	cables: Vec<Cable>,
	modules: HashMap<ModuleName, Module>,
}

fn add_cables(
	circuit: &mut Circuit,
	bytes: &[u8],
	from_conjunction: Option<ModuleName>,
) -> (usize, usize) {
	let out_start_index = circuit.cables.len();

	for out_name in bytes
		.split(|b| *b == b',')
		.map(|other| ModuleName(other[1..].to_vec()))
	{
		circuit.cables.push(Cable {
			to: out_name,
			from_conjunction: from_conjunction.clone(),
		});
	}

	let out_end_index = circuit.cables.len();

	(out_start_index, out_end_index)
}

const RX_NAME: &[u8] = b"rx";
fn rx_conj_trigger_cables(circuit: &Circuit) -> Vec<usize> {
	let rx_name = ModuleName(RX_NAME.to_vec());
	// let mut triggers = vec![];

	let Some(rx_cable) = circuit.cables.iter().find(|c| c.to == rx_name) else {
		return vec![];
	};

	let Cable {
		from_conjunction: Some(con_name),
		..
	} = rx_cable
	else {
		panic!("expected conjunction to lead to rx");
	};

	let Some(Module::Conjunction { ins, .. }) = circuit.modules.get(con_name) else {
		unreachable!();
	};

	ins.iter().map(|(cable_index, _)| *cable_index).collect()
}

pub fn solve(input: &[u8]) -> Solution {
	let mut circuit = Circuit::default();

	// cables[0] is broadcast
	circuit.cables.push(Cable {
		from_conjunction: None,
		to: ModuleName(BROADCASTER_NAME.to_vec()),
	});

	for line in trim_end_newline(input).split(|b| *b == b'\n') {
		let (name, module) = match line[0] {
			b'b' => {
				let name = ModuleName(BROADCASTER_NAME.to_vec());
				// circuit.broadcaster_index = index;

				let outs = add_cables(&mut circuit, &line[(BROADCAST_LEN + ARROW_LEN)..], None);

				(name, Module::Broadcaster { outs })
			}
			b'%' => {
				let name = ModuleName(
					line[1..]
						.iter()
						.take_while(|b| **b != b' ')
						.cloned()
						.collect(),
				);

				let outs = add_cables(&mut circuit, &line[(name.0.len() + ARROW_LEN + 1)..], None);

				(name, Module::FlipFlop { toggle: Lo, outs })
			}
			_ => {
				debug_assert_eq!(line[0], b'&');

				let name = ModuleName(
					line[1..]
						.iter()
						.take_while(|b| **b != b' ')
						.cloned()
						.collect(),
				);

				let outs = add_cables(
					&mut circuit,
					&line[(name.0.len() + ARROW_LEN + 1)..],
					Some(name.clone()),
				);

				(name, Module::Conjunction { ins: vec![], outs })
			}
		};

		circuit.modules.insert(name, module);
	}

	// initialize inward cable indices for conjunction modules
	for (cable_index, cable) in circuit.cables.iter().enumerate() {
		if let Some(Module::Conjunction { ins, .. }) = circuit.modules.get_mut(&cable.to) {
			ins.push((cable_index, Lo));
		}
	}

	let rx_trigger_cables = rx_conj_trigger_cables(&circuit);
	let cycles_to_find = rx_trigger_cables.len();
	let mut rx_trigger_cable_cycles: Vec<(usize, Vec<usize>)> = rx_trigger_cables
		.into_iter()
		.map(|cable| (cable, vec![]))
		.collect();

	let mut cycles_found = 0;

	let mut lo_x_hi: Option<usize> = None;
	let mut iteration = 1;

	while (iteration <= 1000 || cycles_found < cycles_to_find) && iteration < 100_000 {
		let mut pulsating_cable_indices: VecDeque<(usize, Pulse)> = VecDeque::from([(0, Lo)]);

		while let Some((cable_index, pulse)) = pulsating_cable_indices.pop_front() {
			match pulse {
				Hi => circuit.hi_count += 1,
				Lo => circuit.lo_count += 1,
			}

			let Cable { to, .. } = &circuit.cables[cable_index];

			let Some(module) = circuit.modules.get_mut(to) else {
				debug_assert!(matches!(&to.0[..], b"rx" | b"output"));

				continue;
			};
			match module {
				Module::Broadcaster { outs } => {
					for cable_index in outs.0..outs.1 {
						pulsating_cable_indices.push_back((cable_index, pulse));
					}
				}
				Module::FlipFlop { toggle, outs } => {
					if pulse == Lo {
						toggle.toggle();
						for cable_index in outs.0..outs.1 {
							pulsating_cable_indices.push_back((cable_index, *toggle));
							if *toggle == Hi {
								if let Some((_, cycles)) = rx_trigger_cable_cycles
									.iter_mut()
									.find(|(c, _)| *c == cable_index)
								{
									if cycles.len() < 2 {
										cycles.push(iteration);
										if cycles.len() == 2 {
											cycles_found += 1;
										}
									}
								}
							}
						}
					}
				}
				Module::Conjunction { ins, outs } => {
					let (_, input_pulse) = ins
						.iter_mut()
						.find(|input| input.0 == cable_index)
						.expect("cable activating conjunction must be in inputs");

					*input_pulse = pulse;

					if pulse == Hi && to.0 == b"rx".to_vec() {
						println!("[index={cable_index}] RX at {iteration}");
					}

					let out_pulse = if ins.iter().all(|(_, pulse)| *pulse == Hi) {
						Lo
					} else {
						Hi
					};

					for cable_index in outs.0..outs.1 {
						pulsating_cable_indices.push_back((cable_index, out_pulse));

						if out_pulse == Hi {
							if let Some((_, cycles)) = rx_trigger_cable_cycles
								.iter_mut()
								.find(|(c, _)| *c == cable_index)
							{
								if cycles.len() < 2 {
									cycles.push(iteration);
									if cycles.len() == 2 {
										cycles_found += 1;
									}
								}
							}
						}
					}
				}
			}
		}

		if iteration == 1000 {
			lo_x_hi = Some(circuit.lo_count * circuit.hi_count);
		}

		iteration += 1;
	}

	if cycles_found < cycles_to_find {
		panic!("couldn't find cycles in 100_000 runs");
	}

	// test assumption that all cycles start at 0

	if rx_trigger_cable_cycles
		.iter()
		.any(|(_, cycle)| (cycle[1] - cycle[0]) != cycle[0])
	{
		panic!("unexpected complex cycles")
	}

	let min_button_pushed_rx = rx_trigger_cable_cycles
		.into_iter()
		.map(|(_, cycle)| cycle[0])
		.reduce(lowest_common_multiple)
		.unwrap_or(0);

	Solution(lo_x_hi.unwrap(), min_button_pushed_rx)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(32000000,0),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(731517480,244178746156661),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
