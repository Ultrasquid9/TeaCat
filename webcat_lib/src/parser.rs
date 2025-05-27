use std::{
	collections::{HashMap, VecDeque},
	vec,
};

use anyhow::Ok;

use crate::{
	lexer::{StringLiteral, Token},
	prelude::*,
	vecdeque,
};

/// A [TokenStream] that has been evaluated into a useable structure.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast(pub VecDeque<AstNode>);

/// A single node within an [Ast].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstNode {
	Text(String),
	Tag(Tag),
	Var(Var),
	AccessVar(usize, String),
	Macr(Macr),
	AccessMacr(usize, Vec<Var>, String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag {
	pub name: String,
	pub attributes: Attributes,
	pub contents: Ast,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Var {
	pub name: String,
	pub contents: Ast,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Macr {
	pub name: String,
	pub args: Vec<String>,
	pub contents: Ast,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Attributes(pub HashMap<String, StringLiteral>);

impl Ast {
	pub fn empty() -> Self {
		vecdeque![].into()
	}

	pub fn parse(mut tokenstream: TokenStream) -> CatResult<Self> {
		Self::parse_until(&mut tokenstream, None)
	}

	fn parse_until(tokenstream: &mut TokenStream, until: Option<Token>) -> CatResult<Self> {
		let mut nodes = vec![];
		let mut current_line = 0;

		while !tokenstream.0.is_empty() {
			let (line, token) = tokenstream
				.0
				.pop_front()
				.expect("TokenStream should not be empty");

			current_line = line;

			if matches!(until, Some(ref t) if *t == token) {
				return Ok(Self(nodes.into()));
			}

			nodes.push(match token {
				Token::Andpersand => var(tokenstream)?,
				Token::Colon => tag(tokenstream)?,
				Token::Macr => macr(tokenstream)?,
				Token::At => access_macr(tokenstream)?,

				// The remaining tokens are either text themselves or only useful if
				// explicitly required by another, so they can be safely converted
				// into text.
				other => AstNode::Text(format!("{other}")),
			});
		}

		if let Some(token) = until {
			Err(WebCatError::EarlyEof(current_line, token).into())
		} else {
			Ok(Self(nodes.into()))
		}
	}
}

impl From<VecDeque<AstNode>> for Ast {
	fn from(value: VecDeque<AstNode>) -> Self {
		Self(value)
	}
}

impl AstNode {
	pub fn text(str: &str) -> Self {
		Self::Text(str.into())
	}
}

impl Var {
	pub fn new(name: String, tokenstream: &mut TokenStream) -> CatResult<Self> {
		Ok(Self {
			name,
			contents: Ast::parse_until(tokenstream, Some(Token::SemiColon))?,
		})
	}
}

impl Attributes {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	fn parse(tokenstream: &mut TokenStream) -> CatResult<Self> {
		let mut attributes = HashMap::new();
		let mut current_line = 0;

		loop {
			let Some((line, token)) = tokenstream.pop() else {
				return Err(WebCatError::EarlyEof(current_line, Token::CloseBrace).into());
			};
			current_line = line;

			match token {
				Token::CloseBrace => break,
				Token::Text(key) => {
					tokenstream.expect_with_err(
						Token::Colon,
						WebCatError::UnexpectedAttr,
						|| WebCatError::EarlyEof(line, Token::Colon),
					)?;

					let val = match tokenstream.pop() {
						Some((_, Token::Stringliteral(val))) => val,

						Some((line, token)) => {
							return Err(WebCatError::UnexpectedAttr(line, token).into());
						}

						_ => {
							return Err(WebCatError::EarlyEof(
								line,
								Token::Stringliteral(StringLiteral::empty('"')),
							)
							.into());
						}
					};

					attributes.insert(key, val);
				}

				other => {
					return Err(WebCatError::UnexpectedAttr(line, other).into());
				}
			}
		}

		Ok(Self(attributes))
	}
}

impl From<HashMap<String, StringLiteral>> for Attributes {
	fn from(hashmap: HashMap<String, StringLiteral>) -> Self {
		Self(hashmap)
	}
}

fn macr(tokenstream: &mut TokenStream) -> CatResult<AstNode> {
	tokenstream.expect(Token::At)?;
	let (line, name) = tokenstream.current_ident()?;
	tokenstream.expect(Token::OpenBrace)?;

	let mut macr = Macr {
		name,
		args: vec![],
		contents: Ast(vecdeque![]),
	};

	loop {
		let Some((line, token)) = tokenstream.pop() else {
			return Err(WebCatError::EarlyEof(line, Token::CloseBrace).into());
		};

		match token {
			Token::CloseBrace => break,
			Token::Andpersand => {
				let (_, name) = tokenstream.current_ident()?;
				macr.args.push(name);
			}

			other => return Err(WebCatError::UnexpectedToken(line, other).into()),
		}
	}

	tokenstream.expect(Token::OpenBracket)?;

	macr.contents = Ast::parse_until(tokenstream, Some(Token::CloseBracket))?;
	Ok(AstNode::Macr(macr))
}

fn access_macr(tokenstream: &mut TokenStream) -> CatResult<AstNode> {
	let (line, name) = tokenstream.current_ident()?;
	tokenstream.expect(Token::OpenBracket)?;

	let mut vars = vec![];

	loop {
		let Some((line, token)) = tokenstream.pop() else {
			return Err(WebCatError::EarlyEof(line, Token::CloseBracket).into());
		};

		match token {
			Token::CloseBracket => break,
			Token::Andpersand => {
				let AstNode::Var(var) = var(tokenstream)? else {
					todo!("error handling")
				};

				vars.push(var);
			}
			other => return Err(WebCatError::EarlyEof(line, other).into()),
		}
	}

	Ok(AstNode::AccessMacr(line, vars, name))
}

fn var(tokenstream: &mut TokenStream) -> CatResult<AstNode> {
	let (line, name) = tokenstream.current_ident()?;

	// DONT pop from front until we know that the token is one we want
	Ok(if let Some((_, Token::Walrus)) = tokenstream.0.front() {
		// Now we know that it's safe to remove
		tokenstream.pop();
		AstNode::Var(Var::new(name, tokenstream)?)
	} else {
		AstNode::AccessVar(line, name)
	})
}

fn tag(tokenstream: &mut TokenStream) -> CatResult<AstNode> {
	let (_, name) = tokenstream.current_ident()?;

	let mut attributes = Attributes::new();
	if let Some((_, Token::OpenBrace)) = tokenstream.0.front() {
		tokenstream.pop();
		attributes = Attributes::parse(tokenstream)?;
	}

	tokenstream.expect(Token::OpenBracket)?;
	Ok(AstNode::Tag(Tag {
		name,
		attributes,
		contents: Ast::parse_until(tokenstream, Some(Token::CloseBracket))?,
	}))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		lexer::{Token, TokenStream},
		vecdeque,
	};

	#[test]
	fn variables() {
		let tokenstream = vecdeque![
			Token::Andpersand,
			Token::Ident("x".into()),
			Token::Walrus,
			Token::Text(" X".into()),
			Token::SemiColon,
			Token::Andpersand,
			Token::Ident("x".into()),
		];

		let ast = Ast::parse(tokenstream.into()).unwrap();

		assert_eq!(
			ast,
			vecdeque![
				AstNode::Var(Var {
					name: "x".into(),
					contents: vecdeque![AstNode::Text(" X".into())].into()
				}),
				AstNode::AccessVar(0, "x".into()),
			]
			.into()
		);
	}

	#[test]
	fn tags() {
		let tokenstream = vecdeque![
			Token::Colon,
			Token::Ident("a".into()),
			Token::OpenBracket,
			Token::CloseBracket,
			Token::Colon,
			Token::Ident("b".into()),
			Token::OpenBracket,
			Token::CloseBracket,
		];

		let ast = Ast::parse(tokenstream.into()).unwrap();

		assert_eq!(
			ast,
			vecdeque![
				AstNode::Tag(Tag {
					name: "a".into(),
					attributes: Attributes::new(),
					contents: Ast::empty()
				}),
				AstNode::Tag(Tag {
					name: "b".into(),
					attributes: Attributes::new(),
					contents: Ast::empty()
				}),
			]
			.into()
		);
	}

	#[test]
	fn nested_tags() {
		let tokenstream = vecdeque![
			Token::Colon,
			Token::Ident("a".into()),
			Token::OpenBracket,
			Token::Colon,
			Token::Ident("b".into()),
			Token::OpenBracket,
			Token::CloseBracket,
			Token::CloseBracket,
			Token::Colon,
			Token::Ident("c".into()),
			Token::OpenBracket,
			Token::CloseBracket,
		];

		let ast = Ast::parse(tokenstream.into()).unwrap();

		assert_eq!(
			ast,
			vecdeque![
				AstNode::Tag(Tag {
					name: "a".into(),
					attributes: Attributes::new(),
					contents: vecdeque![AstNode::Tag(Tag {
						name: "b".into(),
						attributes: Attributes::new(),
						contents: Ast::empty()
					}),]
					.into()
				}),
				AstNode::Tag(Tag {
					name: "c".into(),
					attributes: Attributes::new(),
					contents: Ast::empty()
				}),
			]
			.into()
		);
	}

	#[test]
	fn attributes() {
		let ast = Ast::parse(TokenStream::lex(":tag{x:\"1\"y:\"2\"}[]")).unwrap();

		assert_eq!(
			ast,
			vecdeque![AstNode::Tag(Tag {
				name: "tag".into(),
				attributes: HashMap::from([
					("x".to_string(), "1".into()),
					("y".to_string(), "2".into()),
				])
				.into(),
				contents: Ast::empty(),
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
		";

		let ast = Ast::parse(TokenStream::lex(str)).unwrap();

		assert_eq!(
			ast,
			vecdeque![
				AstNode::Var(Var {
					name: "title".into(),
					contents: vecdeque![AstNode::Tag(Tag {
						name: "title".into(),
						attributes: Attributes::new(),
						contents: vecdeque![AstNode::text("My Webpage")].into()
					})]
					.into()
				}),
				AstNode::Tag(Tag {
					name: "head".into(),
					attributes: Attributes::new(),
					contents: vecdeque![AstNode::AccessVar(2, "title".into())].into()
				}),
				AstNode::Tag(Tag {
					name: "body".into(),
					attributes: Attributes::new(),
					contents: vecdeque![AstNode::Tag(Tag {
						name: "p".into(),
						attributes: Attributes::new(),
						contents: vecdeque![AstNode::text("&title")].into()
					})]
					.into()
				})
			]
			.into()
		);
	}
}
