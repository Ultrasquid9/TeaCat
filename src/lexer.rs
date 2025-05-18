use std::mem::replace;

pub type TokenTree = Vec<Token>;

#[derive(Debug, Clone)]
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

	/// Defining a variable with the given name
	Var(String),
	/// The start of the contents of a variable
	StartVar,
	/// The end of the contents of a variable
	EndVar,
	/// Accessing a variable with the given name  
	AccessVar(String),
}

impl Token {
	fn string_mut(&mut self) -> Option<&mut String> {
		Some(match self {
			Self::Tag(str)
			| Self::Text(str)
			| Self::Attribute(str)
			| Self::Var(str)
			| Self::AccessVar(str) => str,

			_ => return None,
		})
	}

	fn push_char(&mut self, ch: char) {
		if let Some(str) = self.string_mut() {
			str.push(ch);
		}
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
	let mut is_var = false;
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
			',' if matches!(current, Token::Attribute(_)) => {
				token_switcheroo!(Token::Attribute("".into()))
			}

			'$' => {
				is_var = true;
				token_switcheroo!(Token::Var("".into()))
			}
			whitespace if whitespace.is_whitespace() && matches!(current, Token::Var(_)) => {
				token_switcheroo!(Token::StartVar);
				token_switcheroo!(Token::Text("".into()))
			}
			whitespace if whitespace.is_whitespace() && matches!(current, Token::AccessVar(_)) => {
				token_switcheroo!(Token::Text("".into()))
			}
			';' if is_var => {
				is_var = false;
				token_switcheroo!(Token::EndVar);
			}
			'&' => token_switcheroo!(Token::AccessVar("".into())),

			other => current.push_char(other),
		}
	}

	tokens.push(current);
	tokens
}
