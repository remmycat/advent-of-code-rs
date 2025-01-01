use std::fmt::Debug;
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct IntRangeSet<T: Debug + Clone + Add + PartialEq + Eq + Ord + From<u8>> {
	pub ranges: Vec<(T, T)>,
}

impl<
		T: Debug + Clone + Copy + Add<Output = T> + Sub<Output = T> + PartialEq + Eq + Ord + From<u8>,
	> IntRangeSet<T>
{
	pub const fn new() -> Self {
		IntRangeSet { ranges: vec![] }
	}

	pub fn new_with_capacity(capacity: usize) -> Self {
		IntRangeSet {
			ranges: Vec::with_capacity(capacity),
		}
	}

	fn start_intersecting(&self, range: (T, T)) -> usize {
		// reminder that partition points invert the usual find logic
		// we want to find the first element where this doesn't hold true
		self.ranges
			.partition_point(|other| other.1 + 1.into() < range.0)
	}

	fn end_intersecting(&self, range: (T, T)) -> usize {
		self.ranges
			.partition_point(|other| other.0 <= range.1 + 1.into())
	}

	pub fn add_range(&mut self, range: (T, T)) {
		debug_assert!(range.0 <= range.1);

		let start_intersecting = self.start_intersecting(range);

		if start_intersecting == self.ranges.len() {
			self.ranges.push(range);
			return;
		}

		let end_intersecting = self.end_intersecting(range);

		// means we have no intersection, but we found a place where we can insert the range
		if start_intersecting == end_intersecting {
			self.ranges.insert(start_intersecting, range);
			return;
		}

		let merged = self
			.ranges
			.drain(start_intersecting..end_intersecting)
			.fold(range, |range, other| {
				(range.0.min(other.0), range.1.max(other.1))
			});

		self.ranges.insert(start_intersecting, merged);
	}

	pub fn add_one(&mut self, element: T) {
		self.add_range((element, element))
	}

	pub fn invert(&mut self, bounds: (T, T)) {
		let start_intersecting = self.start_intersecting(bounds);
		let end_intersecting = self.end_intersecting(bounds);
		//let start_intersecting = self.ranges.partition_point(|other| other.1 < bounds.0);

		if start_intersecting == self.ranges.len() {
			self.ranges.clear();
			self.ranges.push(bounds);
			return;
		}

		// let end_intersecting = self.ranges.partition_point(|other| other.0 <= bounds.1);

		if start_intersecting == end_intersecting {
			self.ranges.clear();
			self.ranges.push(bounds);
			return;
		}

		self.ranges.drain(0..start_intersecting);
		self.ranges.drain(end_intersecting..);

		let before = self.ranges.first().and_then(|first| {
			if first.0 > bounds.0 {
				Some((bounds.0, first.0 - 1.into()))
			} else {
				None
			}
		});

		let after = self.ranges.last().and_then(|last| {
			if last.1 < bounds.1 {
				Some((last.1 + 1.into(), bounds.1))
			} else {
				None
			}
		});

		let mut range_iter = self.ranges.iter_mut().peekable();

		while let (Some(range), Some(next)) = (range_iter.next(), range_iter.peek()) {
			range.0 = range.1 + 1.into();
			range.1 = next.0 - 1.into();
		}

		if let Some(after) = after {
			if let Some(last) = self.ranges.last_mut() {
				last.0 = after.0;
				last.1 = after.1;
			} else {
				self.ranges.push(after)
			}
		} else {
			self.ranges.pop();
		}

		if let Some(before) = before {
			self.ranges.insert(0, before);
		}
	}

	pub fn len(&self) -> T {
		self.ranges
			.iter()
			.fold(0.into(), |acc, r| acc + (r.1 - r.0 + 1.into()))
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0.into()
	}
}

impl<
		T: Debug + Clone + Copy + Add<Output = T> + Sub<Output = T> + PartialEq + Eq + Ord + From<u8>,
	> Default for IntRangeSet<T>
{
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn merges_correctly() {
		let mut set = IntRangeSet::<usize>::new();

		set.add_range((2, 4));
		assert_eq!(set.ranges, vec![(2, 4)]);

		set.add_range((8, 9));
		assert_eq!(set.ranges, vec![(2, 4), (8, 9)]);

		set.add_range((14, 17));
		assert_eq!(set.ranges, vec![(2, 4), (8, 9), (14, 17)]);

		set.add_range((3, 5));
		assert_eq!(set.ranges, vec![(2, 5), (8, 9), (14, 17)]);

		set.add_range((4, 12));
		assert_eq!(set.ranges, vec![(2, 12), (14, 17)]);

		set.add_range((13, 13));
		assert_eq!(set.ranges, vec![(2, 17)]);
	}

	#[test]
	fn inverts_correctly() {
		let mut set = IntRangeSet::<usize>::new();

		set.add_range((2, 4));
		set.invert((0, 6));
		assert_eq!(set.ranges, vec![(0, 1), (5, 6)]);

		set.invert((1, 3));
		assert_eq!(set.ranges, vec![(2, 3)]);

		set.invert((2, 3));
		assert_eq!(set.ranges, vec![]);

		set.invert((0, 20));
		set.add_one(22);
		assert_eq!(set.ranges, vec![(0, 20), (22, 22)]);

		set.add_range((22, 40));
		set.add_one(44);
		set.add_one(50);
		let mut cloned = set.clone();
		cloned.invert((0, 50));
		assert_eq!(cloned.ranges, vec![(21, 21), (41, 43), (45, 49)]);

		let mut cloned = set.clone();
		cloned.invert((0, 51));
		assert_eq!(cloned.ranges, vec![(21, 21), (41, 43), (45, 49), (51, 51)]);

		set.invert((0, 52));
		assert_eq!(set.ranges, vec![(21, 21), (41, 43), (45, 49), (51, 52)]);
	}
}
