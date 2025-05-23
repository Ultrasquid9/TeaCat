use std::fmt::{Display, Formatter, Result};

use super::{DARK, DEFAULT};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct Lines([Option<(usize, String)>; 3]);

impl Lines {
	pub fn new(num: usize, text: impl Into<String>) -> Self {
		let text = "\n".to_string() + &text.into();
		let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();
		let mut out = Self::default();

		if let Some(i) = num.checked_sub(1) {
			if let Some(line) = lines.get(i) {
				out.0[0] = Some((i, line.into()));
			}
		}
		if let Some(line) = lines.get(num) {
			out.0[1] = Some((num, line.into()));
		}
		if let Some(line) = lines.get(num + 1) {
			out.0[2] = Some((num + 1, line.into()));
		}

		out
	}
}

impl Display for Lines {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let mut out = String::new();

		for line in &self.0 {
			out.push_str(&if let Some((num, str)) = line {
				format!("{DARK}{num:3} | {DEFAULT}{str}\n")
			} else {
				format!("{DARK}    |{DEFAULT}\n")
			});
		}

		f.write_str(&out)
	}
}
