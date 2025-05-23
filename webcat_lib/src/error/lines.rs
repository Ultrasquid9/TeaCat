use std::fmt::{Display, Formatter, Result};

use super::{DARK, DEFAULT};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
	pub number: usize,
	pub text: String,
}

impl Line {
	pub fn new(number: usize, text: impl Into<String>) -> Self {
		Self {
			number,
			text: text.into(),
		}
	}
}

impl Display for Line {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.write_str(&format!(
			"{DARK}{} | {DEFAULT}{}",
			self.number + 1,
			self.text
		))
	}
}

impl Default for Line {
	fn default() -> Self {
		Self::new(0, "")
	}
}
