use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OperationParsingError {
	#[error("{0} is not a valid opcode")]
	InvalidOpCode(usize),
	#[error("encountered invalid negative op {0}")]
	NegativeOpCode(isize),
	#[error("opcode {opcode:?} has impossible parameter mode {mode:?} for param {position}")]
	ImpossibleParameterMode {
		opcode: OpCode,
		mode: ParameterMode,
		position: usize,
	},
	#[error("Too many parameter modes for opcode (Found {found}, expected {expected})")]
	TooManyParameterModes { found: usize, expected: usize },
	#[error("op {op} includes invalid parameter mode {mode}")]
	InvalidParameterMode { op: usize, mode: usize },
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
	Add,
	Multiply,
	Input,
	Output,
	JumpIfTrue,
	JumpIfFalse,
	LessThan,
	Equals,
	Halt,
}

impl TryFrom<usize> for OpCode {
	type Error = OperationParsingError;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::Add),
			2 => Ok(Self::Multiply),
			3 => Ok(Self::Input),
			4 => Ok(Self::Output),
			5 => Ok(Self::JumpIfTrue),
			6 => Ok(Self::JumpIfFalse),
			7 => Ok(Self::LessThan),
			8 => Ok(Self::Equals),
			99 => Ok(Self::Halt),
			_ => Err(OperationParsingError::InvalidOpCode(value)),
		}
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParameterMode {
	Position,
	Immediate,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
	Add([ParameterMode; 3]),
	Multiply([ParameterMode; 3]),
	Input([ParameterMode; 1]),
	Output([ParameterMode; 1]),
	JumpIfTrue([ParameterMode; 2]),
	JumpIfFalse([ParameterMode; 2]),
	LessThan([ParameterMode; 3]),
	Equals([ParameterMode; 3]),
	Halt,
}

fn get_n_modes<const N: usize>(
	modes: Vec<ParameterMode>,
) -> Result<[ParameterMode; N], OperationParsingError> {
	let mut result = [ParameterMode::Position; N];

	let modes_len = modes.len();
	if modes_len > N {
		return Err(OperationParsingError::TooManyParameterModes {
			found: modes_len,
			expected: N,
		});
	}

	for (i, mode) in modes.into_iter().enumerate() {
		result[i] = mode;
	}

	Ok(result)
}

fn assert_last_is_position<const N: usize>(
	opcode: OpCode,
	params: [ParameterMode; N],
) -> Result<(), OperationParsingError> {
	if params[N - 1] != ParameterMode::Position {
		Err(OperationParsingError::ImpossibleParameterMode {
			opcode,
			mode: params[N - 1],
			position: N,
		})
	} else {
		Ok(())
	}
}

impl TryFrom<usize> for Operation {
	type Error = OperationParsingError;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		let opcode = OpCode::try_from(value % 100)?;

		let mut param_modes = vec![];

		let mut param_remainder = value / 100;

		while param_remainder > 0 {
			let mode = param_remainder % 10;
			let parsed_mode = match mode {
				0 => ParameterMode::Position,
				1 => ParameterMode::Immediate,
				_ => {
					return Err(OperationParsingError::InvalidParameterMode { op: value, mode });
				}
			};

			param_modes.push(parsed_mode);
			param_remainder /= 10;
		}

		match opcode {
			OpCode::Add => {
				let params = get_n_modes::<3>(param_modes)?;
				assert_last_is_position(opcode, params)?;

				Ok(Self::Add(params))
			}
			OpCode::Multiply => {
				let params = get_n_modes::<3>(param_modes)?;
				assert_last_is_position(opcode, params)?;

				Ok(Self::Multiply(params))
			}
			OpCode::Input => {
				let params = get_n_modes::<1>(param_modes)?;
				assert_last_is_position(opcode, params)?;

				Ok(Self::Input(params))
			}
			OpCode::Output => {
				let params = get_n_modes::<1>(param_modes)?;

				Ok(Self::Output(params))
			}
			OpCode::JumpIfTrue => {
				let params = get_n_modes::<2>(param_modes)?;

				Ok(Self::JumpIfTrue(params))
			}
			OpCode::JumpIfFalse => {
				let params = get_n_modes::<2>(param_modes)?;

				Ok(Self::JumpIfFalse(params))
			}
			OpCode::LessThan => {
				let params = get_n_modes::<3>(param_modes)?;
				assert_last_is_position(opcode, params)?;

				Ok(Self::LessThan(params))
			}
			OpCode::Equals => {
				let params = get_n_modes::<3>(param_modes)?;
				assert_last_is_position(opcode, params)?;

				Ok(Self::Equals(params))
			}
			OpCode::Halt => {
				// assert that we don't have any parm_modes
				get_n_modes::<0>(param_modes)?;

				Ok(Self::Halt)
			}
		}
	}
}

impl TryFrom<isize> for Operation {
	type Error = OperationParsingError;

	fn try_from(value: isize) -> Result<Self, Self::Error> {
		if value >= 0 {
			Operation::try_from(value as usize)
		} else {
			Err(OperationParsingError::NegativeOpCode(value))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ParameterMode::*;

	#[test]
	fn simple_ops() {
		assert_eq!(
			Operation::try_from(1_usize),
			Ok(Operation::Add([Position, Position, Position]))
		);
		assert_eq!(
			Operation::try_from(2_usize),
			Ok(Operation::Multiply([Position, Position, Position]))
		);
		assert_eq!(
			Operation::try_from(3_usize),
			Ok(Operation::Input([Position]))
		);
		assert_eq!(
			Operation::try_from(4_usize),
			Ok(Operation::Output([Position]))
		);
		assert_eq!(Operation::try_from(99_usize), Ok(Operation::Halt));

		assert_eq!(
			Operation::try_from(0_usize),
			Err(OperationParsingError::InvalidOpCode(0))
		);
	}

	#[test]
	fn negative_codes() {
		assert_eq!(
			Operation::try_from(1_isize),
			Ok(Operation::Add([Position, Position, Position]))
		);

		assert_eq!(
			Operation::try_from(-1_isize),
			Err(OperationParsingError::NegativeOpCode(-1))
		);
	}

	#[test]
	fn parameter_modes() {
		assert_eq!(
			Operation::try_from(101_usize),
			Ok(Operation::Add([Immediate, Position, Position]))
		);

		assert_eq!(
			Operation::try_from(1001_usize),
			Ok(Operation::Add([Position, Immediate, Position]))
		);

		assert_eq!(
			Operation::try_from(1101_usize),
			Ok(Operation::Add([Immediate, Immediate, Position]))
		);

		assert_eq!(
			Operation::try_from(11101_usize),
			Err(OperationParsingError::ImpossibleParameterMode {
				opcode: OpCode::Add,
				mode: ParameterMode::Immediate,
				position: 3
			})
		);

		assert_eq!(
			Operation::try_from(101101_usize),
			Err(OperationParsingError::TooManyParameterModes {
				found: 4,
				expected: 3,
			})
		);

		assert_eq!(
			Operation::try_from(201_usize),
			Err(OperationParsingError::InvalidParameterMode { op: 201, mode: 2 })
		);
	}
}
