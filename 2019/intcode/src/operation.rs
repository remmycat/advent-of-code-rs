use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OperationParsingError {
	#[error("{0} is not a valid opcode")]
	InvalidOpCode(i64),
	#[error("encountered invalid negative op {0}")]
	NegativeOpCode(i64),
	#[error("Found too many parameter modes for OpCode {opcode} in (expected {expected} parameters, but found additional remainder '{modes_remainder}')")]
	TooManyParameterModes {
		opcode: i64,
		modes_remainder: i64,
		expected: usize,
	},
	#[error("Found invalid parameter mode {mode}")]
	InvalidParameterMode { mode: i64 },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParameterMode {
	Position,
	Immediate,
	Relative,
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
	RelativeBaseOffset([ParameterMode; 1]),
	Halt,
}

fn get_n_modes<const N: usize>(
	opcode: i64,
	mut modes_remainder: i64,
) -> Result<[ParameterMode; N], OperationParsingError> {
	let mut result = [ParameterMode::Position; N];

	#[allow(clippy::needless_range_loop)]
	for i in 0..N {
		if modes_remainder == 0 {
			break;
		}

		let mode = modes_remainder % 10;
		let parsed_mode = match mode {
			0 => ParameterMode::Position,
			1 => ParameterMode::Immediate,
			2 => ParameterMode::Relative,
			_ => {
				return Err(OperationParsingError::InvalidParameterMode { mode });
			}
		};

		result[i] = parsed_mode;
		modes_remainder /= 10;
	}

	if modes_remainder != 0 {
		return Err(OperationParsingError::TooManyParameterModes {
			opcode,
			modes_remainder,
			expected: N,
		});
	}

	Ok(result)
}

impl TryFrom<i64> for Operation {
	type Error = OperationParsingError;

	fn try_from(value: i64) -> Result<Self, Self::Error> {
		if value < 0 {
			return Err(OperationParsingError::NegativeOpCode(value));
		}

		let opcode = value % 100;

		let modes_remainder = value / 100;

		let op = match opcode {
			1 => Self::Add(get_n_modes::<3>(opcode, modes_remainder)?),
			2 => Self::Multiply(get_n_modes::<3>(opcode, modes_remainder)?),
			3 => Self::Input(get_n_modes::<1>(opcode, modes_remainder)?),
			4 => Self::Output(get_n_modes::<1>(opcode, modes_remainder)?),
			5 => Self::JumpIfTrue(get_n_modes::<2>(opcode, modes_remainder)?),
			6 => Self::JumpIfFalse(get_n_modes::<2>(opcode, modes_remainder)?),
			7 => Self::LessThan(get_n_modes::<3>(opcode, modes_remainder)?),
			8 => Self::Equals(get_n_modes::<3>(opcode, modes_remainder)?),
			9 => Self::RelativeBaseOffset(get_n_modes::<1>(opcode, modes_remainder)?),
			99 => {
				// assert that we don't have any extra modes
				get_n_modes::<0>(opcode, modes_remainder)?;
				Self::Halt
			}
			_ => return Err(OperationParsingError::InvalidOpCode(value)),
		};

		Ok(op)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ParameterMode::*;

	#[test]
	fn simple_ops() {
		assert_eq!(
			Operation::try_from(1_i64),
			Ok(Operation::Add([Position, Position, Position]))
		);
		assert_eq!(
			Operation::try_from(2_i64),
			Ok(Operation::Multiply([Position, Position, Position]))
		);
		assert_eq!(Operation::try_from(3_i64), Ok(Operation::Input([Position])));
		assert_eq!(
			Operation::try_from(4_i64),
			Ok(Operation::Output([Position]))
		);
		assert_eq!(Operation::try_from(99_i64), Ok(Operation::Halt));

		assert_eq!(
			Operation::try_from(0_i64),
			Err(OperationParsingError::InvalidOpCode(0))
		);
	}

	#[test]
	fn negative_codes() {
		assert_eq!(
			Operation::try_from(1_i64),
			Ok(Operation::Add([Position, Position, Position]))
		);

		assert_eq!(
			Operation::try_from(-1_i64),
			Err(OperationParsingError::NegativeOpCode(-1))
		);
	}

	#[test]
	fn parameter_modes() {
		assert_eq!(
			Operation::try_from(101_i64),
			Ok(Operation::Add([Immediate, Position, Position]))
		);

		assert_eq!(
			Operation::try_from(1001_i64),
			Ok(Operation::Add([Position, Immediate, Position]))
		);

		assert_eq!(
			Operation::try_from(1101_i64),
			Ok(Operation::Add([Immediate, Immediate, Position]))
		);

		assert_eq!(
			Operation::try_from(1201_i64),
			Ok(Operation::Add([Relative, Immediate, Position]))
		);

		// ParameterMode::Immediate is impossible for the Add operation but
		// allowed for now in here - it will be caught in lib.rs afterwards.
		assert_eq!(
			Operation::try_from(11101_i64),
			Ok(Operation::Add([Immediate, Immediate, Immediate]))
		);

		assert_eq!(
			Operation::try_from(21101_i64),
			Ok(Operation::Add([Immediate, Immediate, Relative]))
		);

		assert_eq!(
			Operation::try_from(101101_i64),
			Err(OperationParsingError::TooManyParameterModes {
				opcode: 1,
				modes_remainder: 1,
				expected: 3,
			})
		);

		assert_eq!(
			Operation::try_from(301_i64),
			Err(OperationParsingError::InvalidParameterMode { mode: 3 })
		);
	}
}
