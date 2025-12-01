#[inline]
pub fn trim_end_newline(input: &[u8]) -> &[u8] {
	if let Some(b'\n') = input.last() {
		&input[..(input.len() - 1)]
	} else {
		input
	}
}
