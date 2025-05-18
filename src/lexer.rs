use std::mem::replace;

pub type TokenTree = Vec<Token>;

#[derive(Debug)]
pub enum Token {
	None,
	Text(String),
	Tag(String),
	EndTag,
}

impl Token {
	fn string_mut(&mut self) -> Option<&mut String> {
		Some(match self {
			Self::Tag(str) => str,
			Self::Text(str) => str,
			_ => return None,
		})
	}

	fn push_char(&mut self, ch: char) {
		if let Some(str) = self.string_mut() {
			str.push(ch);
		}
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
			':' => token_switcheroo!(Token::Tag("".into())),

			'[' => {
				match current {
					Token::Tag(_) => token_switcheroo!(Token::Text("".into())),
					_ => panic!()
				}
			}
			']' => token_switcheroo!(Token::EndTag),

			other => current.push_char(other),
		}
	}

	tokens.push(current);
	tokens
}
