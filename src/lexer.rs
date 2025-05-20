use std::collections::VecDeque;

use nom::error::Error as NomError;

use nom::bytes::complete::tag;

pub type TokenStream = VecDeque<Token>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
	Ident(String),
	Text(String),

	OpenBracket,
	CloseBracket,
	OpenBrace,
	CloseBrace,

	Walrus,
	Colon,
	SemiColon,
	Andpersand,
}

impl Token {
	fn empty() -> Self {
		Self::Text("".into())
	}

	fn string_mut(&mut self) -> Option<&mut String> {
		Some(match self {
			Self::Text(str) | Self::Ident(str) => str,

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

pub fn lex(mut input: String) -> TokenStream {
	let mut tokenstream = vec![];
	let mut current = Token::empty();

	macro_rules! token_switcheroo {
		($new:expr) => {{
			let token = std::mem::replace(&mut current, $new);
			tokenstream.push(token);
		}};
	}

	let mut escaped = false;

	while !input.is_empty() {
		// Handling the backslash escape
		// TODO: \n, \t, etc
		if escaped {
			current.push_char(input.remove(0));
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

		// Checks for one of the operators/keywords is present
		if let Some(token) = rules(&mut input) {
			token_switcheroo!(token);
			continue;
		}

		// No operators/keywords found, so the current char is inserted into the current token
		let ch = input.remove(0);

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
			token_switcheroo!(Token::empty())
		}

		current.push_char(ch);
	}

	// Adding the current token
	tokenstream.push(current);
	// Removing empty tokens
	tokenstream = tokenstream
		.iter()
		.filter(|token| !matches!(token, Token::Text(t) if t.trim().is_empty()))
		.cloned()
		.collect();

	tokenstream.into()
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
	];

	for (key, token) in rules {
		if tag::<&str, &str, NomError<_>>(key)(input).is_ok() {
			*input = input.replacen(key, "", 1);
			return Some(token);
		}
	}

	None
}

fn str_starts_with(input: &str, pat: &str) -> bool {
	tag::<&str, &str, NomError<_>>(pat)(input).is_ok()
}

mod tests {
	#[allow(unused)]
	use super::*;
	#[allow(unused)]
	use crate::vecde;

	#[test]
	fn variables() {
		let str = "
		&x := X;
		&x
		"
		.to_string();

		let tokenstream = lex(str);

		assert_eq!(
			tokenstream,
			vecde![
				Token::Andpersand,
				Token::Ident("x".into()),
				Token::Walrus,
				Token::Text(" X".into()),
				Token::SemiColon,
				Token::Andpersand,
				Token::Ident("x".into()),
			]
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

		let tokenstream = lex(str);

		assert_eq!(
			tokenstream,
			vecde![
				Token::Andpersand,
				Token::Ident("x".into()),
				Token::Text("\n\t\t&x\n\t\t".into()),
			]
		);
	}

	#[test]
	fn multi_tags() {
		let str = "
		:a[]
		:b[]
		"
		.to_string();

		let tokenstream = lex(str);

		assert_eq!(
			tokenstream,
			vecde![
				Token::Colon,
				Token::Ident("a".into()),
				Token::OpenBracket,
				Token::CloseBracket,
				Token::Colon,
				Token::Ident("b".into()),
				Token::OpenBracket,
				Token::CloseBracket,
			]
		);
	}
}
