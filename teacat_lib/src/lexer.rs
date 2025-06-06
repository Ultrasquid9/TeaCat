use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};
use std::mem::replace;

use crate::{prelude::*, vecdeque};

mod str_walker;

const QUOTES: &[char] = &['\'', '"'];

type Rules<T> = &'static [(&'static str, T)];

/// A list of [Tokens](Token) built from a TeaCat string.
#[derive(PartialEq, Eq, Debug)]
pub struct TokenStream(pub VecDeque<(usize, Token)>);

/// The basic building blocks of a TeaCat file.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
	Ident(String),
	Text(String),
	Escape(Escape),
	Stringliteral(StringLiteral),

	OpenBracket,
	CloseBracket,
	OpenBrace,
	CloseBrace,

	Macr,
	Walrus,
	Colon,
	SemiColon,
	Andpersand,
	At,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Escape {
	/// \n
	Newline,
	/// \r
	CarriageReturn,
	/// \t
	Tab,
	// TODO: Unicode character escape
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StringLiteral {
	pub quotes: char,
	pub content: String,
}

impl TokenStream {
	/// Lexes a [str] into a list of [Tokens](Token).
	pub fn lex(input: impl AsRef<str>) -> Self {
		let mut walker = str_walker::StrWalker::new(input.as_ref());
		let mut tokenstream = Self::default();
		let mut current = (0, Token::empty());

		let mut comment_nesting = 0usize;
		let mut escaped = false;

		macro_rules! token_switcheroo {
			($t:expr) => {
				let current = &mut current;
				let token = replace(current, (current.0, $t));
				tokenstream.push(token);
			};
		}

		while !walker.reached_end() {
			if walker.currently_starts_with("\n") {
				current.0 += 1;
			}

			// Handling the backslash escape
			// TODO: \n, \t, etc
			if escaped {
				if let Some(esc) = walker.try_each(Escape::RULES) {
					token_switcheroo!(Token::Escape(esc));
				} else {
					tokenstream.push_current_ch(&mut walker, &mut current);
				}
				escaped = false;
				continue;
			}
			if walker.currently_starts_with("\\") {
				walker.jump_by("\\".len());
				escaped = true;
				continue;
			}

			// Multi-line comments
			if walker.currently_starts_with("<#") {
				walker.jump_by(2);
				comment_nesting += 1;
				continue;
			} else if walker.currently_starts_with("#>") {
				walker.jump_by(2);
				comment_nesting -= 1;
				continue;
			} else if comment_nesting > 0 {
				walker.next_char();
				continue;
			}

			// Single-line comments
			if walker.currently_starts_with("#") {
				walker.jump_to_next("\n");
				continue;
			}

			// Handling string literals
			if matches!(current, (_, Token::Stringliteral(_))) {
				tokenstream.push_current_ch(&mut walker, &mut current);
				continue;
			}

			// Checks for one of the operators/keywords is present
			if let Some(token) = walker.try_each(Token::RULES) {
				token_switcheroo!(token);
				continue;
			}

			// No operators/keywords found, so the current char is inserted into the current token
			tokenstream.push_current_ch(&mut walker, &mut current);
		}

		// Adding the current token
		tokenstream.push(current);
		// Removing empty tokens
		tokenstream.clean_tokens();

		tokenstream
	}

	/// Removes the first [char] of the given [String] and inserts it into the
	/// current [Token] if possible, or creates a new token if not.
	fn push_current_ch(&mut self, input: &mut str_walker::StrWalker, current: &mut (usize, Token)) {
		let ch = input
			.next_char()
			.expect("Should not yet have reached the end!");

		macro_rules! token_switcheroo {
			($t:expr) => {
				let token = replace(current, (current.0.clone(), $t));
				self.push(token);
			};
		}

		// Handling String Literals
		if QUOTES.contains(&ch) && !matches!(current, (_, Token::Stringliteral(_))) {
			token_switcheroo!(Token::Stringliteral(StringLiteral::empty(ch)));
			return;
		}
		if matches!(current, (_, Token::Stringliteral(str)) if str.quotes == ch) {
			token_switcheroo!(Token::empty());
			return;
		}

		// If the current Token does not store text, set it to one that does.
		// - If the current Token implies an Ident, create one.
		// - Otherwise, create a Text.
		if current.1.string_mut().is_none() {
			let token = if current.1.creates_ident() {
				Token::Ident("".into())
			} else {
				Token::empty()
			};
			token_switcheroo!(token);
		}
		// An Ident cannot contain whitespace.
		if matches!(current, (_, Token::Ident(_))) && ch.is_whitespace() {
			token_switcheroo!(Token::empty());
		}

		current.1.push_char(ch);
	}

	/// Converts any sequences of whitespace within [Tokens](Token) into singular spaces, and
	/// removes any tokens consisting only of whitespace.
	fn clean_tokens(&mut self) {
		self.0
			.retain(|(_, token)| !matches!(token.string_ref(), Some(str) if str.trim().is_empty()));

		fn whitespace_check(ch: char) -> bool {
			let ignore = ['\n', '\t'];
			ch.is_whitespace() && !ignore.contains(&ch)
		}

		for (_, token) in &mut self.0 {
			if let Some(str) = token.string_mut() {
				let starts_with = str.starts_with(whitespace_check);
				let ends_with = str.ends_with(whitespace_check);

				*str = str.split_whitespace().collect::<Vec<&str>>().join(" ");

				if starts_with {
					str.insert(0, ' ');
				}
				if ends_with {
					str.push(' ');
				}
			}
		}
	}

	/// Checks to see if the [TokenStream] begins with an ident. If it does, return the name of the
	/// ident and the line number. Otherwise, return a relevant [Error](TeaCatError).
	pub(crate) fn current_ident(&mut self) -> CatResult<(usize, String)> {
		match self.pop() {
			Some((line, Token::Ident(name))) => Ok((line, name)),
			Some((line, token)) => Err(TeaCatError::ExpectedIdent(line, token).into()),
			None => Err(TeaCatError::EarlyEof(0, Token::Ident("ident".into())).into()),
		}
	}

	/// Checks to see if the [TokenStream] begins with the provided [Token]. If it does not, returns
	/// a relevant [Error](TeaCatError).
	pub fn expect(&mut self, token: Token) -> CatResult<()> {
		match self.pop() {
			Some((_, t)) if t == token => Ok(()),
			Some((line, token)) => Err(TeaCatError::UnexpectedToken(line, token).into()),
			_ => Err(TeaCatError::EarlyEof(0, token).into()),
		}
	}

	/// Checks to see if the [TokenStream] begins with the provided [Token]. If it does not, returns
	/// the provided [Error](TeaCatError).
	pub fn expect_with_err(
		&mut self,
		token: Token,
		err_some: impl Fn(usize, Token) -> TeaCatError,
		err_none: impl Fn() -> TeaCatError,
	) -> CatResult<()> {
		match self.pop() {
			Some((_, t)) if t == token => Ok(()),
			Some((line, token)) => Err(err_some(line, token).into()),
			_ => Err(err_none().into()),
		}
	}

	/// Gets a [VecDeque] containing only tokens, without any line numbers.
	pub fn tokens(&self) -> VecDeque<Token> {
		self.0.iter().map(|(_, token)| token.clone()).collect()
	}

	/// Inserts a [Token] and its line number into the back of a [TokenStream].
	pub fn push(&mut self, val: (usize, Token)) {
		self.0.push_back(val);
	}

	/// Removes and returns the front [Token] and its line number from a [TokenStream] (if present).
	pub fn pop(&mut self) -> Option<(usize, Token)> {
		self.0.pop_front()
	}
}

impl Default for TokenStream {
	fn default() -> Self {
		vecdeque![].into()
	}
}

impl From<VecDeque<Token>> for TokenStream {
	fn from(value: VecDeque<Token>) -> Self {
		Self(value.iter().map(|token| (0, token.clone())).collect())
	}
}

impl Token {
	pub const RULES: Rules<Self> = &[
		("[", Token::OpenBracket),
		("]", Token::CloseBracket),
		("{", Token::OpenBrace),
		("}", Token::CloseBrace),
		("macr", Token::Macr),
		(":= ", Token::Walrus),
		(":=", Token::Walrus),
		(":", Token::Colon),
		(";", Token::SemiColon),
		("&", Token::Andpersand),
		("@", Token::At),
	];

	fn empty() -> Self {
		Self::Text("".into())
	}

	fn string_ref(&self) -> Option<&str> {
		Some(match self {
			Self::Text(str) | Self::Ident(str) => str,
			Self::Stringliteral(strlit) => &strlit.content,

			_ => return None,
		})
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
		matches!(self, Self::Colon | Self::Andpersand | Self::At)
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let out: String;

		f.write_str(match self {
			Self::Stringliteral(strlit) => {
				out = strlit.into_string();
				&out
			}
			Self::Text(str) | Self::Ident(str) => str,
			Self::Escape(e) => return e.fmt(f),
			Self::Andpersand => "&",
			Self::CloseBrace => "}",
			Self::CloseBracket => "]",
			Self::Colon => ":",
			Self::OpenBrace => "{",
			Self::OpenBracket => "[",
			Self::SemiColon => ";",
			Self::Walrus => ":=",
			Self::At => "@",
			Self::Macr => " macr ",
		})
	}
}

impl Escape {
	pub const RULES: Rules<Self> = &[
		("r", Self::CarriageReturn),
		("n", Self::Newline),
		("t", Self::Tab),
	];
}

impl Display for Escape {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.write_str(match self {
			Self::CarriageReturn => "\r",
			Self::Newline => "\n",
			Self::Tab => "\t",
		})
	}
}

impl StringLiteral {
	pub fn new(quotes: char, content: String) -> Self {
		Self { quotes, content }
	}

	pub fn empty(quotes: char) -> Self {
		Self::new(quotes, String::new())
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::vecdeque;

	#[test]
	fn variables() {
		let str = "
		&x := X;
		&x
		";

		let tokenstream = TokenStream::lex(str);

		assert_eq!(
			tokenstream.tokens(),
			vecdeque![
				Token::Andpersand,
				Token::Ident("x".into()),
				Token::Walrus,
				Token::Text("X".into()),
				Token::SemiColon,
				Token::Andpersand,
				Token::Ident("x".into()),
			]
		);
	}

	#[test]
	fn escape() {
		let str = "
		&x
		\\&x
		";

		let tokenstream = TokenStream::lex(str);

		assert_eq!(
			tokenstream.tokens(),
			vecdeque![
				Token::Andpersand,
				Token::Ident("x".into()),
				Token::Text("&x".into()),
			]
		);
	}

	#[test]
	fn multi_tags() {
		let str = "
		:a[]
		:b[]
		";

		let tokenstream = TokenStream::lex(str);

		assert_eq!(
			tokenstream.tokens(),
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
		);
	}

	#[test]
	fn attributes() {
		let tokenstream = TokenStream::lex(":tag{x:\"1\" y:'2'}[]");

		assert_eq!(
			tokenstream.tokens(),
			vecdeque![
				Token::Colon,
				Token::Ident("tag".into()),
				Token::OpenBrace,
				Token::Text("x".into()),
				Token::Colon,
				Token::Stringliteral(StringLiteral::new('"', "1".into())),
				Token::Text(" y".into()),
				Token::Colon,
				Token::Stringliteral(StringLiteral::new('\'', "2".into())),
				Token::CloseBrace,
				Token::OpenBracket,
				Token::CloseBracket,
			]
		)
	}

	#[test]
	fn strlit() {
		assert_eq!(
			TokenStream::lex("'input'").tokens(),
			vecdeque![Token::Stringliteral(StringLiteral::new(
				'\'',
				"input".into()
			))]
		)
	}

	#[test]
	fn whitespace() {
		assert_eq!(
			TokenStream::lex("a\ta").tokens(),
			vecdeque![Token::Text("a a".into())]
		);
	}

	#[test]
	fn macros() {
		assert_eq!(
			TokenStream::lex("@person[&name := Juni; &pronouns := she/her;]").tokens(),
			vecdeque![
				Token::At,
				Token::Ident("person".into()),
				Token::OpenBracket,
				Token::Andpersand,
				Token::Ident("name".into()),
				Token::Walrus,
				Token::Text("Juni".into()),
				Token::SemiColon,
				Token::Andpersand,
				Token::Ident("pronouns".into()),
				Token::Walrus,
				Token::Text("she/her".into()),
				Token::SemiColon,
				Token::CloseBracket,
			]
		);
	}

	#[test]
	fn final_boss() {
		// Simplified version of the test.tcat example
		let str = "
		&title := :title[My Webpage];
		:head[&title]
		:body[:p[\\&title]]
		";

		let tokenstream = TokenStream::lex(str);

		assert_eq!(
			tokenstream.tokens(),
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
		);
	}
}
