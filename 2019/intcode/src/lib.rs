use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntCodeError {
	#[error("Address {address} is out of bounds for memory of size {memory_size}")]
	AddressOutOfBounds { address: usize, memory_size: usize },
	#[error("{0} is not a valid op code")]
	InvalidOpCode(usize),
	#[error("usize overflow")]
	USizeOverflow,
	#[error("failed to parse int as usize")]
	ParseInt(#[from] ParseIntError),
	#[error("Ran out of code without halting on an opcode 99")]
	UnexpectedHalt,
}

pub enum OpCode {
	Add,
	Multiply,
	Halt,
}

impl TryFrom<usize> for OpCode {
	type Error = IntCodeError;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::Add),
			2 => Ok(Self::Multiply),
			99 => Ok(Self::Halt),
			_ => Err(IntCodeError::InvalidOpCode(value)),
		}
	}
}

pub enum State {
	Running,
	Halted,
}

pub struct IntCodeProgram {
	/// The initial program
	code: Vec<usize>,
	/// The active program in-memory
	memory: Vec<usize>,
}

impl IntCodeProgram {
	pub fn read(&self, address: usize) -> Result<usize, IntCodeError> {
		self.memory
			.get(address)
			.copied()
			.ok_or(IntCodeError::AddressOutOfBounds {
				address,
				memory_size: self.memory.len(),
			})
	}

	pub fn read_n<const N: usize>(&self, start_address: usize) -> Result<[usize; N], IntCodeError> {
		let mut result = [0; N];

		// Not sure if the lint makes sense or not.
		#[allow(clippy::needless_range_loop)]
		for i in 0..N {
			result[i] = self.read(start_address + i)?
		}
		Ok(result)
	}

	pub fn write(&mut self, address: usize, value: usize) -> Result<(), IntCodeError> {
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
	}

	pub fn run(&mut self) -> Result<(), IntCodeError> {
		let mut active_address = 0;

		while active_address < self.memory.len() {
			match self.execute(active_address)? {
				State::Running => {
					active_address += 4;
				}
				State::Halted => return Ok(()),
			}
		}

		Err(IntCodeError::UnexpectedHalt)
	}

	fn execute(&mut self, address: usize) -> Result<State, IntCodeError> {
		let op_code = OpCode::try_from(self.read(address)?)?;

		match op_code {
			OpCode::Add => {
				let [a_adr, b_adr, res_adr] = self.read_n::<3>(address + 1)?;
				let a = self.read(a_adr)?;
				let b = self.read(b_adr)?;
				let result = a.checked_add(b).ok_or(IntCodeError::USizeOverflow)?;
				self.write(res_adr, result)?;
				Ok(State::Running)
			}
			OpCode::Multiply => {
				let [a_adr, b_adr, res_adr] = self.read_n::<3>(address + 1)?;
				let a = self.read(a_adr)?;
				let b = self.read(b_adr)?;
				let result = a.checked_mul(b).ok_or(IntCodeError::USizeOverflow)?;
				self.write(res_adr, result)?;
				Ok(State::Running)
			}
			OpCode::Halt => Ok(State::Halted),
		}
	}
}

impl FromStr for IntCodeProgram {
	type Err = IntCodeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let code: Vec<usize> = s
			.trim()
			.split(',')
			.map(|int_str| int_str.parse::<usize>())
			.collect::<Result<Vec<usize>, ParseIntError>>()?;

		Ok(IntCodeProgram {
			memory: code.clone(),
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
		program.run()?;

		assert_eq!(program.code, vec![2, 0, 0, 0, 99]);
		Ok(())
	}

	#[test]
	fn mult() -> Result<(), IntCodeError> {
		let input = "2,3,0,3,99";
		let mut program = IntCodeProgram::from_str(input)?;
		program.run()?;

		assert_eq!(program.code, vec![2, 3, 0, 6, 99]);
		Ok(())
	}

	#[test]
	fn mult2() -> Result<(), IntCodeError> {
		let input = "2,4,4,5,99,0";
		let mut program = IntCodeProgram::from_str(input)?;
		program.run()?;

		assert_eq!(program.code, vec![2, 4, 4, 5, 99, 9801]);
		Ok(())
	}

	#[test]
	fn add_mult() -> Result<(), IntCodeError> {
		let input = "1,9,10,3,2,3,11,0,99,30,40,50";
		let mut program = IntCodeProgram::from_str(input)?;
		program.run()?;

		assert_eq!(
			program.code,
			vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
		);
		Ok(())
	}

	#[test]
	fn add_mult2() -> Result<(), IntCodeError> {
		let input = "1,1,1,4,99,5,6,0,99";
		let mut program = IntCodeProgram::from_str(input)?;
		program.run()?;

		assert_eq!(program.code, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
		Ok(())
	}
}
