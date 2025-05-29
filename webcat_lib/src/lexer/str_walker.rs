use super::Rules;

/// Traverses a [str] without needing any reallocations.
pub struct StrWalker<'input> {
	to_walk: &'input str,
	index: usize,
}

impl<'input> StrWalker<'input> {
	/// Creates a new [StrWalker] from the provided [str].
	pub fn new(str: &'input str) -> Self {
		Self {
			to_walk: str,
			index: 0,
		}
	}

	/// Increases the internal index of the [StrWalker], returning the current [char] in the process.
	pub fn next_char(&mut self) -> Option<char> {
		if self.reached_end() {
			return None;
		}

		let og_index = self.index;
		self.index += 1;

		loop {
			if self.reached_end() || self.to_walk.is_char_boundary(self.index) {
				return Some(
					self.to_walk[og_index..self.index]
						.chars()
						.next()
						.expect("Should be a valid char!"),
				);
			}

			self.index += 1;
		}
	}

	/// Whether or not the internal index has reached or passed the length of the [str].
	pub fn reached_end(&self) -> bool {
		self.to_walk.len() <= self.index
	}

	/// Jumps to the next instance of the target.
	pub fn jump_to_next(&mut self, target: &str) {
		loop {
			self.next_char();

			if self.currently_starts_with(target) || self.reached_end() {
				return;
			}
		}
	}

	/// Checks if the internal index is at the start of a pattern matching the target [str].
	pub fn currently_starts_with(&self, cmp: &str) -> bool {
		// Using wrapping_add, then checking the result, seems to be faster than
		// just using a normal add (likely because it doesn't panic)
		let end_index = self.index.wrapping_add(cmp.len());

		if end_index > self.to_walk.len() || end_index < self.index {
			return false;
		}

		self.to_walk.as_bytes()[self.index..end_index] == *cmp.as_bytes()
	}

	/// Jumps forwards by the provided amount of bytes.
	/// # Panics
	/// Panics if the index does not land on a valid char boundary.
	pub fn jump_by(&mut self, amount: usize) {
		self.index += amount;

		if !self.to_walk.is_char_boundary(self.index) {
			panic!("{} is not a valid char boundary", self.index)
		}
	}

	/// Checks each "rule" to see if the [StrWalker] starts with it. If it does,
	/// increase the internal index and return the associated item.
	pub fn try_each<T: Clone>(&mut self, rules: Rules<T>) -> Option<T> {
		for (key, val) in rules {
			if self.currently_starts_with(key) {
				self.jump_by(key.len());
				return Some(val.clone());
			}
		}

		None
	}
}
