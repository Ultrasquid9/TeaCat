use std::collections::{HashMap, VecDeque};

use crate::{
	lexer::{StringLiteral, Token, TokenStream},
	vecdeque,
};

/// A [TokenStream] that has been evaluated into a useable structure.
#[derive(Debug, PartialEq, Eq)]
pub struct Ast(pub VecDeque<AstNode>);

/// A single node within an [Ast].
#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
	Text(String),
	Tag(Tag),
	Var(Var),
	AccessVar(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tag {
	pub name: String,
	pub attributes: Attributes,
	pub contents: Ast,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Var {
	pub name: String,
	pub contents: Ast,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Attributes(pub HashMap<String, StringLiteral>);

impl Ast {
	pub fn empty() -> Self {
		vecdeque![].into()
	}

	pub fn parse(mut tokenstream: TokenStream) -> Self {
		Self::parse_until(&mut tokenstream, None)
	}

	fn parse_until(tokenstream: &mut TokenStream, until: Option<Token>) -> Self {
		let mut nodes = vec![];

		while !tokenstream.0.is_empty() {
			let Some(token) = tokenstream.0.pop_front() else {
				todo!("Error handling")
			};

			if matches!(until, Some(ref t) if *t == token) {
				break;
			}

			nodes.push(match token {
				Token::Andpersand => var(tokenstream),
				Token::Colon => tag(tokenstream),

				Token::Text(text) => AstNode::Text(text),

				// These tokens are only useful if explicitly required by another,
				// so they can be safely converted into text.
				Token::Ident(ident) => AstNode::Text(ident),
				Token::Stringliteral(strlit) => AstNode::Text(strlit.into_string()),
				Token::OpenBracket => AstNode::text("["),
				Token::CloseBracket => AstNode::text("]"),
				Token::OpenBrace => AstNode::text("{"),
				Token::CloseBrace => AstNode::text("}"),
				Token::SemiColon => AstNode::text(";"),
				Token::Walrus => AstNode::text(":="),
				Token::Comma => AstNode::text(","),
			});
		}

		Self(nodes.into())
	}
}

impl From<VecDeque<AstNode>> for Ast {
	fn from(value: VecDeque<AstNode>) -> Self {
		Self(value)
	}
}

impl AstNode {
	fn text(str: &str) -> Self {
		Self::Text(str.into())
	}
}

impl Var {
	fn new(name: String, tokenstream: &mut TokenStream) -> Self {
		Self {
			name,
			contents: Ast::parse_until(tokenstream, Some(Token::SemiColon)),
		}
	}
}

impl Attributes {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	fn parse(tokenstream: &mut TokenStream) -> Self {
		let mut attributes = HashMap::new();

		// Adding a comma here makes it easier to parse
		tokenstream.0.push_front(Token::Comma);

		loop {
			let Some(token) = tokenstream.0.pop_front() else {
				todo!("Error Handling")
			};

			match token {
				Token::CloseBrace => break,
				Token::Comma => {
					let key = match tokenstream.0.pop_front() {
						Some(Token::Text(key)) => key.trim().to_string(),
						Some(Token::CloseBrace) => break,
						other => {
							println!("Unexpected input in attributes: {other:?}");
							todo!("Error Handling");
						}
					};
					let Some(Token::Colon) = tokenstream.0.pop_front() else {
						todo!("Error Handling");
					};
					let val = match tokenstream.0.pop_front() {
						Some(Token::Stringliteral(val)) => val,
						other => {
							println!("Unexpected input in attributes: {other:?}");
							todo!("Error Handling");
						}
					};

					attributes.insert(key, val);
				}
				other => {
					println!("Unexpected input in attributes: {other:?}");
					todo!("Error Handling");
				}
			}
		}

		Self(attributes)
	}
}

impl From<HashMap<String, StringLiteral>> for Attributes {
	fn from(hashmap: HashMap<String, StringLiteral>) -> Self {
		Self(hashmap)
	}
}

fn var(tokenstream: &mut TokenStream) -> AstNode {
	let Some(Token::Ident(name)) = tokenstream.0.pop_front() else {
		return AstNode::text("&");
	};

	// DONT pop from front until we know that the token is one we want
	if let Some(Token::Walrus) = tokenstream.0.front() {
		// Now we know that it's safe to remove
		tokenstream.0.pop_front();
		AstNode::Var(Var::new(name, tokenstream))
	} else {
		AstNode::AccessVar(name)
	}
}

fn tag(tokenstream: &mut TokenStream) -> AstNode {
	let Some(Token::Ident(name)) = tokenstream.0.pop_front() else {
		return AstNode::text(":");
	};

	let mut attributes = Attributes::new();
	if let Some(Token::OpenBrace) = tokenstream.0.front() {
		tokenstream.0.pop_front();
		attributes = Attributes::parse(tokenstream)
	}

	let Some(Token::OpenBracket) = tokenstream.0.pop_front() else {
		return AstNode::Text(":".to_string() + &name);
	};

	AstNode::Tag(Tag {
		name,
		attributes,
		contents: Ast::parse_until(tokenstream, Some(Token::CloseBracket)),
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{lexer::TokenStream, vecdeque};

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

		let ast = Ast::parse(tokenstream.into());

		assert_eq!(
			ast,
			vecdeque![
				AstNode::Var(Var {
					name: "x".into(),
					contents: vecdeque![AstNode::Text(" X".into())].into()
				}),
				AstNode::AccessVar("x".into()),
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

		let ast = Ast::parse(tokenstream.into());

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

		let ast = Ast::parse(tokenstream.into());

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
		let ast = Ast::parse(TokenStream::lex(":tag{x:\"1\", y:\"2\"}[]".into()));

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
		"
		.to_string();

		let ast = Ast::parse(TokenStream::lex(str));

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
					contents: vecdeque![AstNode::AccessVar("title".into())].into()
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
