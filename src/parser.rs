use std::collections::VecDeque;

use crate::{
	lexer::{Token, TokenStream},
	vecde,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Ast(pub VecDeque<AstNode>);

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
	pub attributes: Option<String>,
	pub contents: Ast,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Var {
	pub name: String,
	pub contents: Ast,
}

impl Ast {
	pub fn empty() -> Self {
		Self(vecde![])
	}

	pub fn parse(mut tokenstream: TokenStream) -> Self {
		Self::parse_until(&mut tokenstream, None)
	}

	fn parse_until(tokenstream: &mut TokenStream, until: Option<Token>) -> Self {
		let mut nodes = vec![];

		while !tokenstream.is_empty() {
			let Some(token) = tokenstream.pop_front() else {
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
				Token::OpenBracket => AstNode::text("["),
				Token::CloseBracket => AstNode::text("]"),
				Token::OpenBrace => AstNode::text("{"),
				Token::CloseBrace => AstNode::text("}"),
				Token::SemiColon => AstNode::text(";"),
				Token::Walrus => AstNode::text(":="),
			});
		}

		Self(nodes.into())
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

fn var(tokenstream: &mut TokenStream) -> AstNode {
	let Some(Token::Ident(name)) = tokenstream.pop_front() else {
		return AstNode::text("&");
	};

	if let Some(Token::Walrus) = tokenstream.pop_front() {
		AstNode::Var(Var::new(name, tokenstream))
	} else {
		AstNode::AccessVar(name)
	}
}

fn tag(tokenstream: &mut TokenStream) -> AstNode {
	let Some(Token::Ident(name)) = tokenstream.pop_front() else {
		return AstNode::text(":");
	};
	let Some(Token::OpenBracket) = tokenstream.pop_front() else {
		return AstNode::Text(":".to_string() + &name);
	};

	// TODO: Attributes

	AstNode::Tag(Tag {
		name,
		attributes: None,
		contents: Ast::parse_until(tokenstream, Some(Token::CloseBracket)),
	})
}

mod tests {
	#[allow(unused)]
	use super::*;
	#[allow(unused)]
	use crate::vecde;

	#[test]
	fn variables() {
		let tokenstream = vecde![
			Token::Andpersand,
			Token::Ident("x".into()),
			Token::Walrus,
			Token::Text(" X".into()),
			Token::SemiColon,
			Token::Andpersand,
			Token::Ident("x".into()),
		];

		let ast = Ast::parse(tokenstream);

		assert_eq!(
			ast,
			Ast(vec![
				AstNode::Var(Var {
					name: "x".into(),
					contents: Ast(vec![AstNode::Text(" X".into())].into())
				}),
				AstNode::AccessVar("x".into()),
			]
			.into())
		);
	}

	#[test]
	fn tags() {
		let tokenstream = vecde![
			Token::Colon,
			Token::Ident("a".into()),
			Token::OpenBracket,
			Token::CloseBracket,
			Token::Colon,
			Token::Ident("b".into()),
			Token::OpenBracket,
			Token::CloseBracket,
		];

		let ast = Ast::parse(tokenstream);

		assert_eq!(
			ast,
			Ast(vecde![
				AstNode::Tag(Tag {
					name: "a".into(),
					attributes: None,
					contents: Ast::empty()
				}),
				AstNode::Tag(Tag {
					name: "b".into(),
					attributes: None,
					contents: Ast::empty()
				}),
			])
		);
	}
}
