use std::collections::VecDeque;
use std::mem::replace;
use std::sync::LazyLock;

use regex::Regex;

use crate::vecdeque;

const QUOTES: &[char] = &['\'', '"', '`'];

static WHITESPACE: LazyLock<Regex> =
	LazyLock::new(|| regex::Regex::new("\\s+").expect("Regex should compile"));

/// A list of [Tokens](Token) built from a WebCat string.
#[derive(PartialEq, Eq, Debug)]
pub struct TokenStream(pub VecDeque<Token>);

/// The basic building blocks of a WebCat file.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
	Ident(String),
	Text(String),
	Stringliteral(StringLiteral),

	OpenBracket,
	CloseBracket,
	OpenBrace,
	CloseBrace,

	Walrus,
	Colon,
	SemiColon,
	Andpersand,
	Comma,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StringLiteral {
	pub quotes: char,
	pub content: String,
}

impl TokenStream {
	/// Lexes a [String] into a list of [Tokens](Token).
	pub fn lex(mut input: String) -> Self {
		let mut tokenstream = Self::default();
		let mut current = Token::empty();

		let mut escaped = false;

		while !input.is_empty() {
			// Handling the backslash escape
			// TODO: \n, \t, etc
			if escaped {
				tokenstream.push_current_ch(&mut input, &mut current);
				escaped = false;
				continue;
			}
			if input.starts_with("\\") {
				input.remove(0);
				escaped = true;
				continue;
			}

			// Handling comments
			if input.starts_with("#") {
				if let Some((_, str)) = input.split_once("\n") {
					input = str.into();
				} else {
					input.clear();
				}
				continue;
			}

			// Handling string literals
			if matches!(current, Token::Stringliteral(_)) {
				tokenstream.push_current_ch(&mut input, &mut current);
				continue;
			}

			// Checks for one of the operators/keywords is present
			if let Some(token) = rules(&mut input) {
				let token = replace(&mut current, token);
				tokenstream.push(token);
				continue;
			}

			// No operators/keywords found, so the current char is inserted into the current token
			tokenstream.push_current_ch(&mut input, &mut current);
		}

		// Adding the current token
		tokenstream.push(current);
		// Removing empty tokens
		tokenstream.clean_tokens();

		tokenstream
	}

	/// Removes the first [char] of the given [String] and inserts it into the
	/// current [Token] if possible, or creates a new token if not.
	fn push_current_ch(&mut self, input: &mut String, current: &mut Token) {
		let ch = input.remove(0);

		macro_rules! token_switcheroo {
			($t:expr) => {
				let token = replace(current, $t);
				self.push(token);
			};
		}

		// Handling String Literals
		if QUOTES.contains(&ch) && !matches!(current, Token::Stringliteral(_)) {
			token_switcheroo!(Token::Stringliteral(StringLiteral::new(ch)));
			return;
		}
		if matches!(current, Token::Stringliteral(str) if str.quotes == ch) {
			token_switcheroo!(Token::empty());
			return;
		}

		// If the current Token does not store text, set it to one that does.
		// - If the current Token implies an Ident, create one.
		// - Otherwise, create a Text.
		if current.string_mut().is_none() {
			let token = if current.creates_ident() {
				Token::Ident("".into())
			} else {
				Token::empty()
			};
			token_switcheroo!(token);
		}
		// An Ident cannot contain whitespace.
		if matches!(current, Token::Ident(_)) && ch.is_whitespace() {
			token_switcheroo!(Token::empty());
		}

		current.push_char(ch);
	}

	/// Converts any sequences of whitespace within [Tokens](Token) into singular spaces, and
	/// removes any tokens consisting only of whitespace.
	fn clean_tokens(&mut self) {
		for token in &mut self.0 {
			if let Some(str) = token.string_mut() {
				*str = WHITESPACE
					.replace_all(
						str.trim_matches(|ch: char| ch.is_whitespace() && ch != ' '),
						" ",
					)
					.into();
			}
		}

		self.0 = self
			.0
			.iter()
			.filter(
				|token| !matches!((*token).clone().string_mut(), Some(str) if str.trim().is_empty()),
			)
			.cloned()
			.collect();
	}

	/// Inserts a [Token] into the back of a [TokenStream].
	pub fn push(&mut self, val: Token) {
		self.0.push_back(val);
	}
}

impl Default for TokenStream {
	fn default() -> Self {
		vecdeque![].into()
	}
}

impl From<VecDeque<Token>> for TokenStream {
	fn from(value: VecDeque<Token>) -> Self {
		Self(value)
	}
}

impl Token {
	fn empty() -> Self {
		Self::Text("".into())
	}

	fn string_mut(&mut self) -> Option<&mut String> {
		Some(match self {
			Self::Text(str) | Self::Ident(str) => str,
			Self::Stringliteral(strlit) => &mut strlit.content,

			_ => return None,
		})
	}

	fn push_char(&mut self, ch: char) {
		if let Some(str) = self.string_mut() {
			str.push(ch);
		}
	}

	fn creates_ident(&self) -> bool {
		matches!(self, Self::Colon | Self::Andpersand)
	}
}

impl StringLiteral {
	pub fn new(quotes: char) -> Self {
		Self {
			quotes,
			content: String::new(),
		}
	}

	pub fn into_string(&self) -> String {
		format!("{}{}{}", self.quotes, self.content, self.quotes)
	}
}

impl From<&str> for StringLiteral {
	fn from(value: &str) -> Self {
		Self {
			quotes: '"',
			content: value.to_string(),
		}
	}
}

fn rules(input: &mut String) -> Option<Token> {
	let rules = [
		("[", Token::OpenBracket),
		("]", Token::CloseBracket),
		("{", Token::OpenBrace),
		("}", Token::CloseBrace),
		(":=", Token::Walrus),
		(":", Token::Colon),
		(";", Token::SemiColon),
		("&", Token::Andpersand),
		(",", Token::Comma),
	];

	for (key, token) in rules {
		if input.starts_with(key) {
			*input = input.replacen(key, "", 1);
			return Some(token);
		}
	}

	None
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::vecdeque;

	#[test]
	fn variables() {
		let str = "
		&x := X;
		&x
		"
		.to_string();

		let tokenstream = TokenStream::lex(str);

		assert_eq!(
			tokenstream,
			vecdeque![
				Token::Andpersand,
				Token::Ident("x".into()),
				Token::Walrus,
				Token::Text(" X".into()),
				Token::SemiColon,
				Token::Andpersand,
				Token::Ident("x".into()),
			]
			.into()
		);
	}

	#[test]
	fn rule() {
		assert_eq!(rules(&mut "nothing".into()), None);
		assert_eq!(rules(&mut ":=".into()), Some(Token::Walrus));
		assert_eq!(rules(&mut "ab := cd".into()), None);
	}

	#[test]
	fn escape() {
		let str = "
		&x
		\\&x
		"
		.to_string();

		let tokenstream = TokenStream::lex(str);

		assert_eq!(
			tokenstream,
			vecdeque![
				Token::Andpersand,
				Token::Ident("x".into()),
				Token::Text("&x".into()),
			]
			.into()
		);
	}

	#[test]
	fn multi_tags() {
		let str = "
		:a[]
		:b[]
		"
		.to_string();

		let tokenstream = TokenStream::lex(str);

		assert_eq!(
			tokenstream,
			vecdeque![
				Token::Colon,
				Token::Ident("a".into()),
				Token::OpenBracket,
				Token::CloseBracket,
				Token::Colon,
				Token::Ident("b".into()),
				Token::OpenBracket,
				Token::CloseBracket,
			]
			.into()
		);
	}

	#[test]
	fn attributes() {
		let tokenstream = TokenStream::lex(":tag{x:`1`, y:'2'}[]".into());

		assert_eq!(
			tokenstream,
			vecdeque![
				Token::Colon,
				Token::Ident("tag".into()),
				Token::OpenBrace,
				Token::Text("x".into()),
				Token::Colon,
				Token::Stringliteral(StringLiteral {
					quotes: '`',
					content: "1".into()
				}),
				Token::Comma,
				Token::Text(" y".into()),
				Token::Colon,
				Token::Stringliteral(StringLiteral {
					quotes: '\'',
					content: "2".into()
				}),
				Token::CloseBrace,
				Token::OpenBracket,
				Token::CloseBracket,
			]
			.into(),
		)
	}

	#[test]
	fn strlit() {
		assert_eq!(
			TokenStream::lex("'input'".into()),
			vecdeque![Token::Stringliteral(StringLiteral {
				quotes: '\'',
				content: "input".into()
			})]
			.into()
		)
	}

	#[test]
	fn whitespace() {
		assert_eq!(
			TokenStream::lex("a\ta".into()),
			vecdeque![Token::Text("a a".into())].into()
		);
	}

	#[test]
	fn commas() {
		assert_eq!(
			TokenStream::lex("Hello, World!".into()),
			vecdeque![
				Token::Text("Hello".into()),
				Token::Comma,
				Token::Text(" World!".into()),
			]
			.into()
		);
	}

	#[test]
	fn final_boss() {
		// Simplified version of the main.rs example
		let str = "
		&title := :title[My Webpage];
		:head[&title]
		:body[:p[\\&title]]
		"
		.to_string();

		let tokenstream = TokenStream::lex(str);

		assert_eq!(
			tokenstream,
			vecdeque![
				// Line 1
				Token::Andpersand,
				Token::Ident("title".into()),
				Token::Walrus,
				Token::Colon,
				Token::Ident("title".into()),
				Token::OpenBracket,
				Token::Text("My Webpage".into()),
				Token::CloseBracket,
				Token::SemiColon,
				// Line 2
				Token::Colon,
				Token::Ident("head".into()),
				Token::OpenBracket,
				Token::Andpersand,
				Token::Ident("title".into()),
				Token::CloseBracket,
				// Line 3
				Token::Colon,
				Token::Ident("body".into()),
				Token::OpenBracket,
				Token::Colon,
				Token::Ident("p".into()),
				Token::OpenBracket,
				Token::Text("&title".into()),
				Token::CloseBracket,
				Token::CloseBracket,
			]
			.into()
		);
	}
}
