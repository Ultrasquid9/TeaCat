use std::mem::replace;

pub type TokenTree = Vec<Token>;

#[derive(Debug)]
pub enum Token {
	/// No element present (ignore when parsing)
	None,
	/// A simple text element
	Text(String),

	/// Defining an HTML tag
	Tag(String),
	/// The start of an HTML tag
	StartTag,
	/// The end of an HTML tag
	EndTag,

	/// Defining a tag attribute
	Attribute(String),
	/// The start of a list of attributes
	StartAttribute,
	/// The end of a list of attributes
	EndAttribute,
}

impl Token {
	fn string_mut(&mut self) -> Option<&mut String> {
		Some(match self {
			Self::Tag(str) => str,
			Self::Text(str) => str,
			Self::Attribute(str) => str,
			_ => return None,
		})
	}

	fn push_char(&mut self, ch: char) {
		if let Some(str) = self.string_mut() {
			str.push(ch);
		}
	}

	fn is_attribute(&self) -> bool {
		matches!(self, Self::Attribute(_))
	}

	fn tag_allowed(&self) -> bool {
		!matches!(
			self,
			Self::Tag(_) | Self::Attribute(_) | Self::StartAttribute | Self::EndAttribute
		)
	}
}

pub fn lex(input: String) -> TokenTree {
	let mut tokens = vec![];
	let mut current = Token::None;

	macro_rules! token_switcheroo {
		($new:expr) => {{
			let token = replace(&mut current, $new);
			tokens.push(token);
		}};
	}

	let mut is_comment = false;
	let mut is_escape = false;

	for ch in input.chars() {
		if is_escape && !is_comment {
			current.push_char(ch); // TODO: Custom handling of newlines & such 
			is_escape = false;
			continue;
		}
		if is_comment {
			if ch == '\n' {
				is_comment = false;
			}
			continue;
		}

		match ch {
			'#' => is_comment = true,
			'\\' => is_escape = true,
			':' if current.tag_allowed() => token_switcheroo!(Token::Tag("".into())),

			'[' => {
				token_switcheroo!(Token::StartTag);
				token_switcheroo!(Token::Text("".into()));
			}
			']' => token_switcheroo!(Token::EndTag),

			'{' => {
				token_switcheroo!(Token::StartAttribute);
				token_switcheroo!(Token::Attribute("".into()))
			}
			'}' => token_switcheroo!(Token::EndAttribute),
			',' if current.is_attribute() => token_switcheroo!(Token::Attribute("".into())),

			other => current.push_char(other),
		}
	}

	tokens.push(current);
	tokens
}
