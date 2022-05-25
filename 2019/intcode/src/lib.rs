pub mod operation;

use fallible_iterator::{FallibleIterator, IntoFallibleIterator};
use operation::{Operation, OperationParsingError, ParameterMode};
use std::{collections::VecDeque, num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum IntCodeError {
	#[error("address {address} is out of bounds for memory of size {memory_size}")]
	AddressOutOfBounds { address: usize, memory_size: usize },
	#[error("encountered negative address {address}")]
	NegativeAddress { address: isize },
	#[error("usize overflow")]
	USizeOverflow,
	#[error("failed to parse int as usize")]
	ParseInt(#[from] ParseIntError),
	#[error("failed to parse operation")]
	ParseOp(#[from] OperationParsingError),
	#[error("ran out of code without halting on an opcode 99")]
	UnexpectedHalt,
	#[error("tried to write to value instead of address")]
	WriteToValue,
	#[error("tried to read address as value")]
	ReadAddressAsValue,
	#[error("program halted but not but {unused} inputs were unused")]
	HaltedWithoutUsingAllInputs { unused: usize },
	#[error("program requested input but no inputs were left")]
	NoInputsLeft,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Int {
	Address(usize),
	Value(isize),
}

impl Int {
	pub fn value(&self) -> Result<isize, IntCodeError> {
		match self {
			Self::Value(value) => Ok(*value),
			Self::Address(_) => Err(IntCodeError::ReadAddressAsValue),
		}
	}

	pub fn read(&self, program: &IntCodeProgramIter) -> Result<isize, IntCodeError> {
		match self {
			Self::Value(value) => Ok(*value),
			Self::Address(address) => program.read(*address, ParameterMode::Immediate)?.value(),
		}
	}

	pub fn write(
		&self,
		program: &mut IntCodeProgramIter,
		value: isize,
	) -> Result<(), IntCodeError> {
		match self {
			Self::Value(_) => Err(IntCodeError::WriteToValue),
			Self::Address(address) => program.write(*address, value),
		}
	}
}

fn try_value_as_adress(value: isize) -> Result<usize, IntCodeError> {
	value
		.try_into()
		.map_err(|_| IntCodeError::NegativeAddress { address: value })
}

#[derive(Debug, PartialEq)]
pub enum HaltReason {
	Input(Int),
	Output(Int),
	Halted,
}

#[derive(Debug, Clone)]
pub struct IntCodeProgram {
	code: Vec<isize>,
	inputs: VecDeque<isize>,
}

impl IntCodeProgram {
	pub fn patch(self, address: usize, value: isize) -> Result<IntCodeProgram, IntCodeError> {
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
				memory_size: self.code.len(),
			})
		}
	}

	pub fn inputs(self, inputs: Vec<isize>) -> IntCodeProgram {
		IntCodeProgram {
			inputs: VecDeque::from(inputs),
			code: self.code,
		}
	}

	pub fn run(self) -> Result<Vec<isize>, IntCodeError> {
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
			.map(|int_str| int_str.parse::<isize>())
			.collect::<Result<Vec<isize>, ParseIntError>>()?;

		Ok(IntCodeProgram {
			inputs: VecDeque::new(),
			code,
		})
	}
}

impl IntoFallibleIterator for IntCodeProgram {
	type Item = isize;
	type Error = IntCodeError;
	type IntoFallibleIter = IntCodeProgramIter;

	fn into_fallible_iter(self) -> Self::IntoFallibleIter {
		IntCodeProgramIter {
			memory: self.code,
			inputs: self.inputs,
			active_address: 0,
		}
	}
}

pub struct IntCodeProgramIter {
	/// The active program in-memory
	memory: Vec<isize>,
	inputs: VecDeque<isize>,
	active_address: usize,
}

impl IntCodeProgramIter {
	pub fn read(&self, address: usize, mode: ParameterMode) -> Result<Int, IntCodeError> {
		let int = self
			.memory
			.get(address)
			.copied()
			.ok_or(IntCodeError::AddressOutOfBounds {
				address,
				memory_size: self.memory.len(),
			})?;

		match mode {
			ParameterMode::Immediate => Ok(Int::Value(int)),
			ParameterMode::Position => Ok(Int::Address(try_value_as_adress(int)?)),
		}
	}

	pub fn read_n<const N: usize>(
		&self,
		start_address: usize,
		modes: [ParameterMode; N],
	) -> Result<[Int; N], IntCodeError> {
		let mut result = [Int::Value(0); N];

		// Not sure if the lint makes sense or not.
		// #[allow(clippy::needless_range_loop)]
		for i in 0..N {
			result[i] = self.read(start_address + i, modes[i])?
		}
		Ok(result)
	}

	pub fn write(&mut self, address: usize, value: isize) -> Result<(), IntCodeError> {
		let memory_size = self.memory.len();

		if address < memory_size {
			self.memory[address] = value;
			Ok(())
		} else {
			Err(IntCodeError::AddressOutOfBounds {
				address,
				memory_size,
			})
		}
	}

	fn execute(&mut self) -> Result<Option<HaltReason>, IntCodeError> {
		let address = self.active_address;
		let operation =
			Operation::try_from(self.read(address, ParameterMode::Immediate)?.value()?)?;

		// println!("Operation: {operation:?}");

		match operation {
			Operation::Add(param_modes) => {
				let [a_int, b_int, res_adr] = self.read_n::<3>(address + 1, param_modes)?;
				let a = a_int.read(self)?;
				let b = b_int.read(self)?;
				let result = a.checked_add(b).ok_or(IntCodeError::USizeOverflow)?;
				res_adr.write(self, result)?;
				self.active_address += 4;
				Ok(None)
			}
			Operation::Multiply(param_modes) => {
				let [a_int, b_int, res_adr] = self.read_n(address + 1, param_modes)?;
				let a = a_int.read(self)?;
				let b = b_int.read(self)?;
				let result = a.checked_mul(b).ok_or(IntCodeError::USizeOverflow)?;
				res_adr.write(self, result)?;
				self.active_address += 4;
				Ok(None)
			}
			Operation::Input(param_modes) => {
				let [addr] = self.read_n(self.active_address + 1, param_modes)?;
				self.active_address += 2;
				Ok(Some(HaltReason::Input(addr)))
			}
			Operation::Output(param_modes) => {
				let [out] = self.read_n(self.active_address + 1, param_modes)?;
				self.active_address += 2;
				Ok(Some(HaltReason::Output(out)))
			}
			Operation::JumpIfTrue(param_modes) => {
				let [check, jumpto] = self.read_n(self.active_address + 1, param_modes)?;

				if check.read(self)? != 0 {
					let jumpto_val: isize = jumpto.read(self)?;
					let new_address: usize = try_value_as_adress(jumpto_val)?;
					self.active_address = new_address;
				} else {
					self.active_address += 3;
				}
				Ok(None)
			}
			Operation::JumpIfFalse(param_modes) => {
				let [check, jumpto] = self.read_n(self.active_address + 1, param_modes)?;

				if check.read(self)? == 0 {
					let jumpto_val: isize = jumpto.read(self)?;
					let new_address: usize = try_value_as_adress(jumpto_val)?;
					self.active_address = new_address;
				} else {
					self.active_address += 3;
				}
				Ok(None)
			}
			Operation::LessThan(param_modes) => {
				let [check_a, check_b, addr] = self.read_n(self.active_address + 1, param_modes)?;

				let result = if check_a.read(self)? < check_b.read(self)? {
					1
				} else {
					0
				};
				addr.write(self, result)?;
				self.active_address += 4;
				Ok(None)
			}
			Operation::Equals(param_modes) => {
				let [check_a, check_b, addr] = self.read_n(self.active_address + 1, param_modes)?;

				let result = if check_a.read(self)? == check_b.read(self)? {
					1
				} else {
					0
				};
				addr.write(self, result)?;
				self.active_address += 4;
				Ok(None)
			}
			Operation::Halt => Ok(Some(HaltReason::Halted)),
		}
	}

	pub fn add_input(&mut self, value: isize) {
		self.inputs.push_back(value);
	}
}

impl FallibleIterator for IntCodeProgramIter {
	type Item = isize;
	type Error = IntCodeError;

	fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
		while self.active_address < self.memory.len() {
			match self.execute()? {
				None => (),
				Some(HaltReason::Input(target)) => {
					if let Some(input) = self.inputs.pop_front() {
						target.write(self, input)?;
					} else {
						return Err(IntCodeError::NoInputsLeft);
					}
				}
				Some(HaltReason::Output(val)) => {
					return Ok(Some(val.read(self)?));
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
}
