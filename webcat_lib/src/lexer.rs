use std::collections::VecDeque;
use std::mem::replace;

use nom::error::Error as NomError;

use nom::bytes::complete::tag;

const QUOTES: &[char] = &['\'', '"', '`'];

#[derive(PartialEq, Eq, Debug)]
pub struct TokenStream(pub VecDeque<Token>);

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
	pub fn lex(mut input: String) -> Self {
		let mut tokenstream = vec![];
		let mut current = Token::empty();

		let mut escaped = false;

		while !input.is_empty() {
			// Handling the backslash escape
			// TODO: \n, \t, etc
			if escaped {
				push_current_ch(&mut input, &mut current, &mut tokenstream);
				escaped = false;
				continue;
			}
			if str_starts_with(&input, "\\") {
				input.remove(0);
				escaped = true;
				continue;
			}

			// Handling comments
			if str_starts_with(&input, "#") {
				if let Some((_, str)) = input.split_once("\n") {
					input = str.into();
				} else {
					input.clear();
				}
				continue;
			}

			// Handling string literals
			if matches!(current, Token::Stringliteral(_)) {
				push_current_ch(&mut input, &mut current, &mut tokenstream);
				continue;
			}

			// Checks for one of the operators/keywords is present
			if let Some(token) = rules(&mut input) {
				let token = replace(&mut current, token);
				tokenstream.push(token);
				continue;
			}

			// No operators/keywords found, so the current char is inserted into the current token
			push_current_ch(&mut input, &mut current, &mut tokenstream);
		}

		// Adding the current token
		tokenstream.push(current);
		// Removing empty tokens
		clean_tokens(&mut tokenstream);

		tokenstream.into()
	}
}

impl From<Vec<Token>> for TokenStream {
	fn from(value: Vec<Token>) -> Self {
		Self(value.into())
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

fn push_current_ch(input: &mut String, current: &mut Token, tokenstream: &mut Vec<Token>) {
	let ch = input.remove(0);

	macro_rules! token_switcheroo {
		($t:expr) => {
			let token = replace(current, $t);
			tokenstream.push(token);
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
		if tag::<&str, &str, NomError<_>>(key)(input).is_ok() {
			*input = input.replacen(key, "", 1);
			return Some(token);
		}
	}

	None
}

/// Checks if the input starts with the provided pattern
fn str_starts_with(input: &str, pat: &str) -> bool {
	tag::<&str, &str, NomError<_>>(pat)(input).is_ok()
}

/// Removing leading and trailing whitespace from tokens
fn clean_tokens(tokens: &mut Vec<Token>) {
	for token in &mut *tokens {
		if let Some(str) = token.string_mut() {
			*str = str.trim().into();
		}
	}

	*tokens = tokens
		.iter()
		.filter(|token| !matches!((*token).clone().string_mut(), Some(str) if str.is_empty()))
		.cloned()
		.collect();
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
				Token::Text("X".into()),
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
		let tokenstream = TokenStream::lex(":tag{x:1, y:2}[]".into());

		assert_eq!(
			tokenstream,
			vecdeque![
				Token::Colon,
				Token::Ident("tag".into()),
				Token::OpenBrace,
				Token::Text("x".into()),
				Token::Colon,
				Token::Ident("1".into()),
				Token::Comma,
				Token::Text("y".into()),
				Token::Colon,
				Token::Ident("2".into()),
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
