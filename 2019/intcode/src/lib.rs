pub mod operation;

use operation::{Operation, OperationParsingError, ParameterMode};
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
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
	#[error("program halted but not only {used} of {expected} inputs were used")]
	HaltedWithoutUsingAllInputs { expected: usize, used: usize },
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

	pub fn read(&self, program: &IntCodeProgram) -> Result<isize, IntCodeError> {
		match self {
			Self::Value(value) => Ok(*value),
			Self::Address(address) => program.read(*address, ParameterMode::Immediate)?.value(),
		}
	}

	pub fn write(&self, program: &mut IntCodeProgram, value: isize) -> Result<(), IntCodeError> {
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

pub struct IntCodeProgram {
	/// The initial program
	code: Vec<isize>,
	/// The active program in-memory
	memory: Vec<isize>,
	active_address: usize,
}

impl IntCodeProgram {
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

	pub fn reset(&mut self) {
		self.memory = self.code.clone();
		self.active_address = 0;
	}

	pub fn run(&mut self, inputs: &[isize]) -> Result<Vec<isize>, IntCodeError> {
		let mut input_index = 0;
		let mut outputs = vec![];

		while self.active_address < self.memory.len() {
			match self.execute()? {
				Some(HaltReason::Halted) => {
					if input_index != inputs.len() {
						return Err(IntCodeError::HaltedWithoutUsingAllInputs {
							expected: inputs.len(),
							used: input_index,
						});
					} else {
						return Ok(outputs);
					}
				}
				Some(HaltReason::Input(target)) => {
					if input_index >= inputs.len() {
						return Err(IntCodeError::NoInputsLeft);
					}

					target.write(self, inputs[input_index])?;
					input_index += 1;
				}
				Some(HaltReason::Output(val)) => {
					outputs.push(val.read(self)?);
				}
				None => (),
			}
		}

		Err(IntCodeError::UnexpectedHalt)
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
			memory: code.clone(),
			active_address: 0,
			code,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add() -> Result<(), IntCodeError> {
		let input = "1,0,0,0,99";
		let mut program = IntCodeProgram::from_str(input)?;

		assert_eq!(program.run(&[])?, vec![]);

		assert_eq!(program.memory, vec![2, 0, 0, 0, 99]);
		Ok(())
	}

	#[test]
	fn mult() -> Result<(), IntCodeError> {
		let input = "2,3,0,3,99";
		let mut program = IntCodeProgram::from_str(input)?;

		assert_eq!(program.run(&[])?, vec![]);

		assert_eq!(program.memory, vec![2, 3, 0, 6, 99]);
		Ok(())
	}

	#[test]
	fn mult2() -> Result<(), IntCodeError> {
		let input = "2,4,4,5,99,0";
		let mut program = IntCodeProgram::from_str(input)?;

		assert_eq!(program.run(&[])?, vec![]);

		assert_eq!(program.memory, vec![2, 4, 4, 5, 99, 9801]);
		Ok(())
	}

	#[test]
	fn add_mult() -> Result<(), IntCodeError> {
		let input = "1,9,10,3,2,3,11,0,99,30,40,50";
		let mut program = IntCodeProgram::from_str(input)?;

		assert_eq!(program.run(&[])?, vec![]);

		assert_eq!(
			program.memory,
			vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
		);
		Ok(())
	}

	#[test]
	fn add_mult2() -> Result<(), IntCodeError> {
		let input = "1,1,1,4,99,5,6,0,99";
		let mut program = IntCodeProgram::from_str(input)?;

		assert_eq!(program.run(&[])?, vec![]);

		assert_eq!(program.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
		Ok(())
	}

	#[test]
	fn add_neg() -> Result<(), IntCodeError> {
		let input = "101,-2,0,0,99";
		let mut program = IntCodeProgram::from_str(input)?;

		assert_eq!(program.run(&[])?, vec![]);

		assert_eq!(program.memory, vec![99, -2, 0, 0, 99]);
		Ok(())
	}

	#[test]
	fn position_equal_io() -> Result<(), IntCodeError> {
		let code = "3,9,8,9,10,9,4,9,99,-1,8";
		// Using position mode, consider whether the input is equal to 8;
		// output 1 (if it is) or 0 (if it is not)

		let mut program = IntCodeProgram::from_str(code)?;

		assert_eq!(program.run(&[8])?, vec![1]);

		program.reset();
		assert_eq!(program.run(&[-3])?, vec![0]);

		Ok(())
	}

	#[test]
	fn position_less_than_io() -> Result<(), IntCodeError> {
		let code = "3,9,7,9,10,9,4,9,99,-1,8";
		// Using position mode, consider whether the input is less than 8;
		// output 1 (if it is) or 0 (if it is not)

		let mut program = IntCodeProgram::from_str(code)?;

		assert_eq!(program.run(&[7])?, vec![1]);

		program.reset();
		assert_eq!(program.run(&[8])?, vec![0]);

		program.reset();
		assert_eq!(program.run(&[200])?, vec![0]);

		Ok(())
	}

	#[test]
	fn immediate_equal_io() -> Result<(), IntCodeError> {
		let code = "3,3,1108,-1,8,3,4,3,99";
		// Using immediate mode, consider whether the input is equal to 8;
		// output 1 (if it is) or 0 (if it is not)

		let mut program = IntCodeProgram::from_str(code)?;

		assert_eq!(program.run(&[8])?, vec![1]);

		program.reset();
		assert_eq!(program.run(&[-3])?, vec![0]);

		Ok(())
	}

	#[test]
	fn immediate_less_than_io() -> Result<(), IntCodeError> {
		let code = "3,3,1107,-1,8,3,4,3,99";
		// Using immediate mode, consider whether the input is less than 8;
		// output 1 (if it is) or 0 (if it is not)

		let mut program = IntCodeProgram::from_str(code)?;

		assert_eq!(program.run(&[7])?, vec![1]);

		program.reset();
		assert_eq!(program.run(&[8])?, vec![0]);

		program.reset();
		assert_eq!(program.run(&[200])?, vec![0]);

		Ok(())
	}

	#[test]
	fn position_jump() -> Result<(), IntCodeError> {
		let code = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
		// Here are some jump tests that take an input,
		// then output 0 if the input was zero or 1 if the input was non-zero

		let mut program = IntCodeProgram::from_str(code)?;

		assert_eq!(program.run(&[0])?, vec![0]);

		program.reset();
		assert_eq!(program.run(&[-13])?, vec![1]);

		Ok(())
	}

	#[test]
	fn immediate_jump() -> Result<(), IntCodeError> {
		let code = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
		// Here are some jump tests that take an input,
		// then output 0 if the input was zero or 1 if the input was non-zero

		let mut program = IntCodeProgram::from_str(code)?;

		assert_eq!(program.run(&[0])?, vec![0]);

		program.reset();
		assert_eq!(program.run(&[-13])?, vec![1]);

		Ok(())
	}
}
