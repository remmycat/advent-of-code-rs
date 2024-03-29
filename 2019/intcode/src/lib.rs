mod operation;

use fallible_iterator::{FallibleIterator, IntoFallibleIterator};
use operation::{Operation, OperationParsingError, ParameterMode};
use std::{collections::VecDeque, num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum IntCodeError {
	// Doesn't have to be an error, we could save code as a Map as well
	#[error("address {address} is out of bounds for code of size {code_size}")]
	AddressOutOfBounds { address: usize, code_size: usize },
	#[error("encountered impossible negative address {address}")]
	NegativeAddress { address: i64 },
	#[error("encountered value that does not fit into a signed 64bit integer")]
	IntOverflow,
	#[error("encountered an address that does not fit into a {} bit address (because we're running on a {}-bit architecture)", usize::BITS, usize::BITS)]
	AddressOverflow,
	#[error("failed to parse int as i64")]
	ParseInt(#[from] ParseIntError),
	#[error("failed to parse operation")]
	ParseOp(#[from] OperationParsingError),
	#[error("ran out of code without halting on an opcode 99")]
	UnexpectedHalt,
	#[error("tried to write to value instead of address")]
	WriteToValue,
	#[error("tried to read address as value")]
	ReadAddressAsValue,
	#[error("program halted but {unused} inputs were unused")]
	HaltedWithoutUsingAllInputs { unused: usize },
	#[error("program requested input but no inputs were left")]
	NoInputsLeft,
	#[error("op includes invalid parameter mode {0:?} for a write-parameter")]
	InvalidWriteParameterMode(ParameterMode),
}

fn try_value_as_address(value: i64) -> Result<usize, IntCodeError> {
	value
		.try_into()
		.map_err(|_| IntCodeError::NegativeAddress { address: value })
}

#[derive(Debug, PartialEq, Eq)]
pub enum HaltReason {
	Input(usize),
	Output(i64),
	Halted,
}

#[derive(Debug, Clone)]
pub struct IntCodeProgram {
	code: Vec<i64>,
	inputs: VecDeque<i64>,
}

impl IntCodeProgram {
	pub fn patch(self, address: usize, value: i64) -> Result<IntCodeProgram, IntCodeError> {
		if address <= self.code.len() {
			let mut new_code = self.code;
			new_code[address] = value;

			Ok(IntCodeProgram {
				code: new_code,
				inputs: self.inputs,
			})
		} else {
			Err(IntCodeError::AddressOutOfBounds {
				address,
				code_size: self.code.len(),
			})
		}
	}

	pub fn inputs(self, inputs: Vec<i64>) -> IntCodeProgram {
		IntCodeProgram {
			inputs: VecDeque::from(inputs),
			code: self.code,
		}
	}

	pub fn run(self) -> Result<Vec<i64>, IntCodeError> {
		let mut outputs = vec![];
		let mut program_iterator = self.into_fallible_iter();

		while let Some(output) = program_iterator.next()? {
			outputs.push(output);
		}

		Ok(outputs)
	}
}

impl FromStr for IntCodeProgram {
	type Err = IntCodeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let code = s
			.trim()
			.split(',')
			.map(|int_str| int_str.parse::<i64>())
			.collect::<Result<Vec<i64>, ParseIntError>>()?;

		Ok(IntCodeProgram {
			inputs: VecDeque::new(),
			code,
		})
	}
}

impl IntoFallibleIterator for IntCodeProgram {
	type Item = i64;
	type Error = IntCodeError;
	type IntoFallibleIter = IntCodeProgramIter;

	fn into_fallible_iter(self) -> Self::IntoFallibleIter {
		IntCodeProgramIter {
			memory: self.code,
			inputs: self.inputs,
			active_address: 0,
			relative_base: 0,
		}
	}
}

pub struct IntCodeProgramIter {
	/// The active program in-memory
	memory: Vec<i64>,
	inputs: VecDeque<i64>,
	active_address: usize,
	relative_base: i64,
}

impl IntCodeProgramIter {
	pub fn read_value(&self, address: usize) -> i64 {
		self.memory.get(address).copied().unwrap_or(0)
	}

	pub fn add_input(&mut self, value: i64) {
		self.inputs.push_back(value);
	}

	fn write(&mut self, address: usize, value: i64) {
		if address >= self.memory.len() {
			let mut padding = vec![0_i64; address - self.memory.len() + 1];
			// We know this cannot panic because we only make space for address
			// and address must already fit into a usize.
			self.memory.append(&mut padding);
		}

		self.memory[address] = value;
	}

	fn get_read_param(&self, offset: usize, mode: ParameterMode) -> Result<i64, IntCodeError> {
		let int = self.read_value(
			self.active_address
				.checked_add(offset)
				.ok_or(IntCodeError::AddressOverflow)?,
		);

		match mode {
			ParameterMode::Immediate => Ok(int),
			ParameterMode::Position => Ok(self.read_value(try_value_as_address(int)?)),
			ParameterMode::Relative => Ok(self.read_value(try_value_as_address(
				int.checked_add(self.relative_base)
					.ok_or(IntCodeError::IntOverflow)?,
			)?)),
		}
	}

	fn get_write_param(&self, offset: usize, mode: ParameterMode) -> Result<usize, IntCodeError> {
		let int = self.read_value(
			self.active_address
				.checked_add(offset)
				.ok_or(IntCodeError::AddressOverflow)?,
		);

		match mode {
			ParameterMode::Position => Ok(try_value_as_address(int)?),
			ParameterMode::Relative => Ok(try_value_as_address(
				int.checked_add(self.relative_base)
					.ok_or(IntCodeError::IntOverflow)?,
			)?),
			other => Err(IntCodeError::InvalidWriteParameterMode(other)),
		}
	}

	fn go_forward(&mut self, increment: usize) -> Result<(), IntCodeError> {
		self.active_address = self
			.active_address
			.checked_add(increment)
			.ok_or(IntCodeError::AddressOverflow)?;
		Ok(())
	}

	fn execute(&mut self) -> Result<Option<HaltReason>, IntCodeError> {
		let operation = Operation::try_from(self.read_value(self.active_address))?;

		// println!("Operation: {operation:?}");

		match operation {
			Operation::Add(modes) => {
				let lhs = self.get_read_param(1, modes[0])?;
				let rhs = self.get_read_param(2, modes[1])?;
				let result_target = self.get_write_param(3, modes[2])?;

				let result = lhs.checked_add(rhs).ok_or(IntCodeError::IntOverflow)?;

				self.write(result_target, result);
				self.go_forward(4)?;
				Ok(None)
			}
			Operation::Multiply(modes) => {
				let lhs = self.get_read_param(1, modes[0])?;
				let rhs = self.get_read_param(2, modes[1])?;
				let result_target = self.get_write_param(3, modes[2])?;

				let result = lhs.checked_mul(rhs).ok_or(IntCodeError::IntOverflow)?;

				self.write(result_target, result);
				self.go_forward(4)?;
				Ok(None)
			}
			Operation::Input(modes) => {
				let input_target = self.get_write_param(1, modes[0])?;

				self.go_forward(2)?;
				Ok(Some(HaltReason::Input(input_target)))
			}
			Operation::Output(modes) => {
				let output_value = self.get_read_param(1, modes[0])?;

				self.go_forward(2)?;
				Ok(Some(HaltReason::Output(output_value)))
			}
			Operation::JumpIfTrue(modes) => {
				let check = self.get_read_param(1, modes[0])?;
				let jump_target = self.get_read_param(2, modes[1])?;

				if check != 0 {
					let new_address: usize = try_value_as_address(jump_target)?;
					self.active_address = new_address;
				} else {
					self.go_forward(3)?;
				}
				Ok(None)
			}
			Operation::JumpIfFalse(modes) => {
				let check = self.get_read_param(1, modes[0])?;
				let jump_target = self.get_read_param(2, modes[1])?;

				if check == 0 {
					let new_address: usize = try_value_as_address(jump_target)?;
					self.active_address = new_address;
				} else {
					self.go_forward(3)?;
				}
				Ok(None)
			}
			Operation::LessThan(modes) => {
				let lhs = self.get_read_param(1, modes[0])?;
				let rhs = self.get_read_param(2, modes[1])?;
				let result_target = self.get_write_param(3, modes[2])?;

				let result = i64::from(lhs < rhs);

				self.write(result_target, result);
				self.go_forward(4)?;
				Ok(None)
			}
			Operation::Equals(modes) => {
				let lhs = self.get_read_param(1, modes[0])?;
				let rhs = self.get_read_param(2, modes[1])?;
				let result_target = self.get_write_param(3, modes[2])?;

				let result = i64::from(lhs == rhs);

				self.write(result_target, result);
				self.go_forward(4)?;
				Ok(None)
			}
			Operation::RelativeBaseOffset(modes) => {
				let offset = self.get_read_param(1, modes[0])?;

				self.relative_base += offset;
				self.go_forward(2)?;
				Ok(None)
			}
			Operation::Halt => Ok(Some(HaltReason::Halted)),
		}
	}
}

impl FallibleIterator for IntCodeProgramIter {
	type Item = i64;
	type Error = IntCodeError;

	fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
		while self.active_address < self.memory.len() {
			match self.execute()? {
				None => (),
				Some(HaltReason::Input(input_target)) => {
					if let Some(input) = self.inputs.pop_front() {
						self.write(input_target, input);
					} else {
						return Err(IntCodeError::NoInputsLeft);
					}
				}
				Some(HaltReason::Output(output_value)) => {
					return Ok(Some(output_value));
				}
				Some(HaltReason::Halted) => {
					return Ok(None);
				}
			}
		}

		Err(IntCodeError::UnexpectedHalt)
	}
}

#[cfg(test)]
mod tests {
	use std::vec;

	use super::*;

	#[test]
	fn add() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("1,0,0,0,99")?;
		let mut instance = program.into_fallible_iter();

		assert_eq!(instance.next()?, None);
		assert_eq!(instance.memory, vec![2, 0, 0, 0, 99]);

		Ok(())
	}

	#[test]
	fn mult() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("2,3,0,3,99")?;
		let mut instance = program.into_fallible_iter();

		assert_eq!(instance.next()?, None);
		assert_eq!(instance.memory, vec![2, 3, 0, 6, 99]);

		Ok(())
	}

	#[test]
	fn mult2() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("2,4,4,5,99,0")?;
		let mut instance = program.into_fallible_iter();

		assert_eq!(instance.next()?, None);
		assert_eq!(instance.memory, vec![2, 4, 4, 5, 99, 9801]);

		Ok(())
	}

	#[test]
	fn add_mult() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("1,9,10,3,2,3,11,0,99,30,40,50")?;
		let mut instance = program.into_fallible_iter();

		assert_eq!(instance.next()?, None);
		assert_eq!(
			instance.memory,
			vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
		);

		Ok(())
	}

	#[test]
	fn add_mult2() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("1,1,1,4,99,5,6,0,99")?;
		let mut instance = program.into_fallible_iter();

		assert_eq!(instance.next()?, None);
		assert_eq!(instance.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

		Ok(())
	}

	#[test]
	fn add_neg() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("101,-2,0,0,99")?;
		let mut instance = program.into_fallible_iter();

		assert_eq!(instance.next()?, None);
		assert_eq!(instance.memory, vec![99, -2, 0, 0, 99]);

		Ok(())
	}

	#[test]
	fn position_equal_io() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("3,9,8,9,10,9,4,9,99,-1,8")?;
		// Using position mode, consider whether the input is equal to 8;
		// output 1 (if it is) or 0 (if it is not)

		assert_eq!(program.clone().inputs(vec![8]).run(), Ok(vec![1]));
		assert_eq!(program.inputs(vec![-3]).run(), Ok(vec![0]));

		Ok(())
	}

	#[test]
	fn position_less_than_io() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("3,9,7,9,10,9,4,9,99,-1,8")?;
		// Using position mode, consider whether the input is less than 8;
		// output 1 (if it is) or 0 (if it is not)

		assert_eq!(program.clone().inputs(vec![7]).run(), Ok(vec![1]));
		assert_eq!(program.clone().inputs(vec![8]).run(), Ok(vec![0]));
		assert_eq!(program.inputs(vec![200]).run(), Ok(vec![0]));

		Ok(())
	}

	#[test]
	fn immediate_equal_io() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("3,3,1108,-1,8,3,4,3,99")?;
		// Using immediate mode, consider whether the input is equal to 8;
		// output 1 (if it is) or 0 (if it is not)

		assert_eq!(program.clone().inputs(vec![8]).run(), Ok(vec![1]));
		assert_eq!(program.inputs(vec![-3]).run(), Ok(vec![0]));

		Ok(())
	}

	#[test]
	fn immediate_less_than_io() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("3,3,1107,-1,8,3,4,3,99")?;
		// Using immediate mode, consider whether the input is less than 8;
		// output 1 (if it is) or 0 (if it is not)

		assert_eq!(program.clone().inputs(vec![7]).run(), Ok(vec![1]));
		assert_eq!(program.clone().inputs(vec![8]).run(), Ok(vec![0]));
		assert_eq!(program.inputs(vec![200]).run(), Ok(vec![0]));

		Ok(())
	}

	#[test]
	fn position_jump() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")?;
		// Here are some jump tests that take an input,
		// then output 0 if the input was zero or 1 if the input was non-zero

		assert_eq!(program.clone().inputs(vec![0]).run(), Ok(vec![0]));
		assert_eq!(program.inputs(vec![-13]).run(), Ok(vec![1]));

		Ok(())
	}

	#[test]
	fn immediate_jump() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")?;
		// Here are some jump tests that take an input,
		// then output 0 if the input was zero or 1 if the input was non-zero

		assert_eq!(program.clone().inputs(vec![0]).run(), Ok(vec![0]));
		assert_eq!(program.inputs(vec![-13]).run(), Ok(vec![1]));

		Ok(())
	}

	#[test]
	fn quine() -> Result<(), IntCodeError> {
		let program =
			IntCodeProgram::from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")?;
		// takes no input and produces a copy of itself as output.

		assert_eq!(
			program.run(),
			Ok(vec![
				109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
			])
		);

		Ok(())
	}

	#[test]
	fn big_numbers() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("1102,34915192,34915192,7,4,7,99,0")?;

		assert_eq!(program.run(), Ok(vec![1219070632396864]));

		Ok(())
	}

	#[test]
	fn big_number_copy() -> Result<(), IntCodeError> {
		let program = IntCodeProgram::from_str("104,1125899906842624,99")?;

		assert_eq!(program.run(), Ok(vec![1125899906842624]));

		Ok(())
	}
}
