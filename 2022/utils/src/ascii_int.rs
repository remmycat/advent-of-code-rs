fn b_digit(b: u8) -> usize {
	if !b.is_ascii_digit() {
		panic!("Not an ascii digit")
	}
	(b - b'0') as usize
}

/// Parse `usize` from ascii digit bytes.
/// Panics on unexpected bytes (non-digits).
pub fn parse_uint(digits: &[u8]) -> usize {
	digits[1..]
		.iter()
		.fold(b_digit(digits[0]), |dig, b| dig * 10 + b_digit(*b))
}

/// Parse `isize` from ascii digit bytes. First byte can be either a digit or `-`.
/// Panics on unexpected bytes (non-minus & non-digit)
pub fn parse_int(digits: &[u8]) -> isize {
	if digits[0] == b'-' {
		-(parse_uint(&digits[1..]) as isize)
	} else {
		parse_uint(digits) as isize
	}
}
