#[derive(Debug, Default, Clone)]
pub struct Range {
	pub start: usize,
	pub end: usize,
}

impl From<(usize, usize)> for Range {
	fn from(value: (usize, usize)) -> Self {
		Self {
			start: value.0,
			end: value.1,
		}
	}
}

impl Range {
	#[inline(always)]
	pub fn intersects_with(&self, other: &Range) -> bool {
		self.start <= other.end && self.end >= other.start
	}

	#[inline(always)]
	pub fn contains(&self, element: usize) -> bool {
		self.start <= element && self.end >= element
	}

	#[inline(always)]
	pub fn span(&self) -> usize {
		self.end - self.start + 1
	}

	#[inline(always)]
	pub fn intersection(&self, other: &Range) -> Option<Range> {
		if self.intersects_with(other) {
			Some(Range {
				start: self.start.max(other.start),
				end: self.end.min(other.end),
			})
		} else {
			None
		}
	}

	#[inline(always)]
	pub fn offset<T: Into<isize>>(&self, offset: T) -> Range {
		let offset = offset.into();
		Range {
			start: (self.start as isize + offset) as usize,
			end: (self.end as isize + offset) as usize,
		}
	}

	#[inline(always)]
	pub fn intersection_with_rest(&self, other: &Range) -> Option<IntersectionWithRest> {
		if !self.intersects_with(other) {
			return None;
		}
		let intersection = Range {
			start: self.start.max(other.start),
			end: self.end.min(other.end),
		};

		let left_rest = if other.start < self.start {
			Some(Range {
				start: other.start,
				end: self.start - 1,
			})
		} else {
			None
		};

		let right_rest = if other.end > self.end {
			Some(Range {
				start: self.end + 1,
				end: other.end,
			})
		} else {
			None
		};

		Some(IntersectionWithRest {
			range: intersection,
			left_rest,
			right_rest,
		})
	}
}

#[derive(Debug)]
pub struct IntersectionWithRest {
	pub range: Range,
	pub left_rest: Option<Range>,
	pub right_rest: Option<Range>,
}
