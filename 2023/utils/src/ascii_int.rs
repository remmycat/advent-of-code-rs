#[inline(always)]
fn b_digit_usize_unchecked(b: u8) -> usize {
	(b - b'0') as usize
}

#[inline(always)]
fn b_digit_usize(b: u8) -> usize {
	if !b.is_ascii_digit() {
		panic!("Not an ascii digit")
	}
	b_digit_usize_unchecked(b)
}

#[inline(always)]
fn b_digit_unchecked(b: u8) -> u8 {
	b - b'0'
}

/// Parse `usize` from ascii digit bytes.
/// Panics on unexpected bytes (non-digits).
#[inline(always)]
pub fn parse_uint(digits: &[u8]) -> usize {
	digits[1..].iter().fold(b_digit_usize(digits[0]), |dig, b| {
		dig * 10 + b_digit_usize(*b)
	})
}

/// Parse `usize` from ascii digit bytes.
/// Panics on unexpected bytes (non-digits).
#[inline(always)]
pub fn parse_uint_unchecked(digits: &[u8]) -> usize {
	digits[1..]
		.iter()
		.fold(b_digit_usize_unchecked(digits[0]), |dig, b| {
			dig * 10 + b_digit_usize_unchecked(*b)
		})
}

/// Parse `u8` from ascii digit bytes.
/// Does not (always) panic on unexpected bytes (non-digits).
#[inline(always)]
pub fn parse_u8_unchecked(digits: &[u8]) -> u8 {
	digits[1..]
		.iter()
		.fold(b_digit_unchecked(digits[0]), |dig, b| {
			dig * 10 + b_digit_unchecked(*b)
		})
}

/// Parse `isize` from ascii digit bytes. First byte can be either a digit or `-`.
/// Panics on unexpected bytes (non-minus & non-digit)
#[inline(always)]
pub fn parse_int(digits: &[u8]) -> isize {
	if digits[0] == b'-' {
		-(parse_uint(&digits[1..]) as isize)
	} else {
		parse_uint(digits) as isize
	}
}
