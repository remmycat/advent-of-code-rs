use aoc_utils::{ascii_int::parse_uint_unchecked, iteration::expect_n, trim::trim_end_newline};

#[derive(Debug, PartialEq, Eq)]
pub struct Solution(usize, usize);

// 5 bits per char, (repr. 0-25, lowercase a-z)
type WorkflowName = usize;
type PartRating = usize;

const NAMES_LEN: usize = 0b11111_11111_11111 + 1;

#[derive(Debug, Clone, Copy)]
enum Register {
	XtremelyCoolLooking,
	Musical,
	Aerodynamic,
	Shiny,
}

impl Register {
	fn from_byte(byte: u8) -> Self {
		debug_assert!(matches!(byte, b'x' | b'm' | b'a' | b's'));
		match byte {
			b'x' => Self::XtremelyCoolLooking,
			b'm' => Self::Musical,
			b'a' => Self::Aerodynamic,
			_ => Self::Shiny,
		}
	}
}

#[derive(Debug, Clone)]
enum JumpTarget {
	Workflow(WorkflowName),
	Accepted,
	Rejected,
}

impl JumpTarget {
	fn parse(bytes: &[u8]) -> Self {
		match bytes[0] {
			b'A' => Self::Accepted,
			b'R' => Self::Rejected,
			_ => Self::Workflow(encode_name(bytes)),
		}
	}
}

#[derive(Debug, Clone)]
enum Instruction {
	JumpIfGreaterThan(Register, PartRating, JumpTarget),
	JumpIfLessThan(Register, PartRating, JumpTarget),
	Jump(JumpTarget),
}

impl Instruction {
	fn from_bytes(bytes: &[u8]) -> Self {
		if bytes.len() <= 3 {
			// Must be unconditional jump
			Self::Jump(JumpTarget::parse(bytes))
		} else {
			let register = Register::from_byte(bytes[0]);
			let [digits, target] = expect_n(
				bytes[2..].split(|b| *b == b':'),
				"must have number and jump target",
			);
			let comparison = parse_uint_unchecked(digits);
			let target = JumpTarget::parse(target);

			debug_assert!(matches!(bytes[1], b'>' | b'<'));

			match bytes[1] {
				b'<' => Self::JumpIfLessThan(register, comparison, target),
				_ => Self::JumpIfGreaterThan(register, comparison, target),
			}
		}
	}
}

fn encode_name(name: &[u8]) -> WorkflowName {
	name.iter().fold(0, |acc, ascii_char| {
		(acc << 5) | (*ascii_char - b'a') as WorkflowName
	})
}

// "in" encoded according to the scheme above
const IN_NAME: usize = 0b01000_01101;

#[derive(Debug)]
struct Part {
	x: PartRating,
	m: PartRating,
	a: PartRating,
	s: PartRating,
}

impl Part {
	fn get_register(&self, reg: Register) -> &PartRating {
		match reg {
			Register::XtremelyCoolLooking => &self.x,
			Register::Musical => &self.m,
			Register::Aerodynamic => &self.a,
			Register::Shiny => &self.s,
		}
	}

	fn from_bytes(bytes: &[u8]) -> Self {
		let [x, m, a, s] = expect_n(
			bytes[1..(bytes.len() - 1)].split(|b| *b == b','),
			"expected all 4 registers",
		);
		Self {
			x: parse_uint_unchecked(&x[2..]),
			m: parse_uint_unchecked(&m[2..]),
			a: parse_uint_unchecked(&a[2..]),
			s: parse_uint_unchecked(&s[2..]),
		}
	}
}

#[derive(Debug, Clone)]
struct PartRange {
	x: (PartRating, PartRating),
	m: (PartRating, PartRating),
	a: (PartRating, PartRating),
	s: (PartRating, PartRating),
}

impl PartRange {
	fn new() -> Self {
		PartRange {
			x: (1, 4000),
			m: (1, 4000),
			a: (1, 4000),
			s: (1, 4000),
		}
	}

	fn register(&self, reg: Register) -> &(PartRating, PartRating) {
		match reg {
			Register::XtremelyCoolLooking => &self.x,
			Register::Musical => &self.m,
			Register::Aerodynamic => &self.a,
			Register::Shiny => &self.s,
		}
	}

	fn register_mut(&mut self, reg: Register) -> &mut (PartRating, PartRating) {
		match reg {
			Register::XtremelyCoolLooking => &mut self.x,
			Register::Musical => &mut self.m,
			Register::Aerodynamic => &mut self.a,
			Register::Shiny => &mut self.s,
		}
	}

	fn distinct_combinations(self) -> usize {
		let x = self.x.1 - self.x.0 + 1;
		let m = self.m.1 - self.m.0 + 1;
		let a = self.a.1 - self.a.0 + 1;
		let s = self.s.1 - self.s.0 + 1;
		x * m * a * s
	}
}

fn find_all_combinations(
	workflows: &[Option<Vec<Instruction>>; NAMES_LEN],
	mut range: PartRange,
	current_workflow: WorkflowName,
	index: usize,
) -> usize {
	let mut combinations = 0;

	for (instruction_index, instruction) in workflows[current_workflow]
		.as_ref()
		.expect("target is valid")
		.iter()
		.enumerate()
		.skip(index)
	{
		let target = match instruction.clone() {
			Instruction::Jump(target) => target,
			Instruction::JumpIfLessThan(reg, comp, target) => {
				let own_range = range.register(reg);
				if own_range.1 < comp {
					// our whole range fulfills condition and jumps
					target
				} else if own_range.0 >= comp {
					// our whole range does not fulfill condition and stays
					continue;
				} else {
					// some of our range does not fulfill condition, other part jumps
					let mut not_jumping_part = range.clone();
					*not_jumping_part.register_mut(reg) = (comp, own_range.1);
					range.register_mut(reg).1 = comp - 1;

					// add all combinations for the range that didn't jump!
					combinations += find_all_combinations(
						workflows,
						not_jumping_part,
						current_workflow,
						instruction_index + 1,
					);

					target
				}
			}
			Instruction::JumpIfGreaterThan(reg, comp, target) => {
				let own_range = range.register(reg);
				if own_range.0 > comp {
					// our whole range fulfills condition and jumps
					target
				} else if own_range.1 <= comp {
					// our whole range does not fulfill condition and stays
					continue;
				} else {
					// some of our range does not fulfill condition, other part jumps
					let mut not_jumping_part = range.clone();
					*not_jumping_part.register_mut(reg) = (own_range.0, comp);
					range.register_mut(reg).0 = comp + 1;

					// add all combinations for the range that didn't jump!
					combinations += find_all_combinations(
						workflows,
						not_jumping_part,
						current_workflow,
						instruction_index + 1,
					);

					target
				}
			}
		};

		combinations += match target {
			// our current ranges are accepted! let's add the combinations
			JumpTarget::Accepted => range.distinct_combinations(),
			JumpTarget::Rejected => 0,
			JumpTarget::Workflow(workflow) => find_all_combinations(workflows, range, workflow, 0),
		};

		break;
	}

	combinations
}

const EMPTY_WORKFLOW: Option<Vec<Instruction>> = None;

pub fn solve(input: &[u8]) -> Solution {
	let mut workflows: [Option<Vec<Instruction>>; NAMES_LEN] = [EMPTY_WORKFLOW; NAMES_LEN];

	let mut line_iter = trim_end_newline(input).split(|b| *b == b'\n');

	// part 1, collect instructions
	for line in line_iter.by_ref().take_while(|line| !line.is_empty()) {
		let brace_pos = line
			.iter()
			.position(|b| *b == b'{')
			.expect("must have opening brace");

		let name = encode_name(&line[0..brace_pos]);

		workflows[name] = Some(
			line[(brace_pos + 1)..(line.len() - 1)]
				.split(|b| *b == b',')
				.map(Instruction::from_bytes)
				.collect(),
		);
	}

	let mut accepted_sum = 0;

	for part in line_iter.map(Part::from_bytes) {
		let mut current_workflow = IN_NAME;

		let accepted = 'outer: loop {
			'inner: for instruction in workflows[current_workflow]
				.as_ref()
				.expect("target is valid")
				.iter()
			{
				let target = match instruction.clone() {
					Instruction::Jump(target) => target,
					Instruction::JumpIfLessThan(reg, comp, target) => {
						if *part.get_register(reg) < comp {
							target
						} else {
							continue 'inner;
						}
					}
					Instruction::JumpIfGreaterThan(reg, comp, target) => {
						if *part.get_register(reg) > comp {
							target
						} else {
							continue 'inner;
						}
					}
				};

				match target {
					JumpTarget::Accepted => break 'outer true,
					JumpTarget::Rejected => break 'outer false,
					JumpTarget::Workflow(workflow) => {
						current_workflow = workflow;
						continue 'outer;
					}
				}
			}

			panic!("Reached end of workflow without jump");
		};

		if accepted {
			accepted_sum += part.x + part.m + part.a + part.s;
		}
	}

	let part_range = PartRange::new();

	let combinations = find_all_combinations(&workflows, part_range, IN_NAME, 0);

	Solution(accepted_sum, combinations)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest]
	#[case::example(
		include_bytes!("../inputs/example.txt"),
		Solution(19114,167409079868000),
	)]
	#[case::personal(
		include_bytes!("../inputs/personal.txt"),
		Solution(280909,116138474394508),
	)]
	fn solution(#[case] input: &[u8], #[case] expected: Solution) {
		assert_eq!(solve(input), expected);
	}
}
