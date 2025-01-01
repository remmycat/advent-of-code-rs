#[inline]
pub fn greatest_common_divisor(x: usize, y: usize) -> usize {
	if y == 0 {
		if x == 0 {
			panic!("tried to find common divisor for 0 and 0");
		}
		x
	} else {
		greatest_common_divisor(y, x % y)
	}
}

#[inline]
pub fn lowest_common_multiple(x: usize, y: usize) -> usize {
	let greatest_common_divisor = greatest_common_divisor(x, y);

	x * (y / greatest_common_divisor)
}

// todo: chinese remainder theorem?
