use std::array;

#[inline(always)]
pub fn expect_n<const N: usize, T: IntoIterator>(
	iter: T,
	expectation: &'static str,
) -> [T::Item; N] {
	let mut iter = iter.into_iter();
	array::from_fn(|_| iter.next().expect(expectation))
}
