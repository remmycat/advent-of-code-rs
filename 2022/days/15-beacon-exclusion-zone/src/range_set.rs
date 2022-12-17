#[derive(Debug)]
pub struct IntRangeSet(Vec<(isize, isize)>);

impl IntRangeSet {
	pub const fn new() -> Self {
		IntRangeSet(vec![])
	}

	pub fn add_range(&mut self, range: (isize, isize)) {
		assert!(range.0 <= range.1);

		// using partition points like this is a bit confusing 😵‍💫
		let start_intersecting = self.0.partition_point(|other| other.1 < range.0 - 1);

		if start_intersecting == self.0.len() {
			self.0.push(range);
			return;
		}

		let end_intersecting = self.0.partition_point(|other| other.0 <= range.1 + 1);

		// means we have no intersection, but we found a place where we can insert the range
		if start_intersecting == end_intersecting {
			self.0.insert(start_intersecting, range);
			return;
		}

		let merged = self
			.0
			.drain(start_intersecting..end_intersecting)
			.fold(range, |range, other| {
				(range.0.min(other.0), range.1.max(other.1))
			});

		self.0.insert(start_intersecting, merged);
	}

	pub fn len(&self) -> usize {
		self.0.iter().map(|r| (r.1 - r.0 + 1) as usize).sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn merges_correctly() {
		let mut set = IntRangeSet::new();

		set.add_range((2, 4));
		assert_eq!(set.0, vec![(2, 4)]);

		set.add_range((8, 9));
		assert_eq!(set.0, vec![(2, 4), (8, 9)]);

		set.add_range((14, 17));
		assert_eq!(set.0, vec![(2, 4), (8, 9), (14, 17)]);

		set.add_range((3, 5));
		assert_eq!(set.0, vec![(2, 5), (8, 9), (14, 17)]);

		set.add_range((4, 12));
		assert_eq!(set.0, vec![(2, 12), (14, 17)]);

		set.add_range((13, 13));
		assert_eq!(set.0, vec![(2, 17)]);
	}
}
