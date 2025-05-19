use nom::error::Error as NomError;

use nom::bytes::complete::tag;

pub type TokenStream = Vec<Token>;

#[derive(PartialEq, Eq, Debug)]
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

	None,
}

impl Token {
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
	let mut current = Token::None;

	macro_rules! token_switcheroo {
		($new:expr) => {{
			let token = std::mem::replace(&mut current, $new);
			tokenstream.push(token);
		}};
	}

	let mut escaped = false;

	while input.len() > 0 {
		// Handling comments
		if str_starts_with(&input, "#") {
			if let Some((_, str)) = input.split_once("\n") {
				input = str.into();
			} else {
				input.clear();
			}
			continue;
		}

		// Handling the backslash escape
		// TODO: \n, \t, etc
		if escaped {
			current.push_char(input.remove(0));
			escaped = false;
			continue;
		}
		if str_starts_with(&input, "\\") {
			escaped = true;
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
				Token::Text("".into())
			};
			token_switcheroo!(token);
		}
		// An Ident cannot contain whitespace.
		if matches!(current, Token::Ident(_)) && ch.is_whitespace() {
			token_switcheroo!(Token::Text("".into()))
		}

		current.push_char(ch);
	}

	tokenstream.push(current);
	tokenstream
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

#[test]
fn test() {
	assert_eq!(rules(&mut "nothing".into()), None);
	assert_eq!(rules(&mut ":=".into()), Some(Token::Walrus));
	assert_eq!(rules(&mut "ab := cd".into()), None);
}
