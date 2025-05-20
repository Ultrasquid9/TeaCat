use std::collections::{HashMap, VecDeque};

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
	pub attributes: HashMap<String, String>,
	pub contents: Ast,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Var {
	pub name: String,
	pub contents: Ast,
}

impl Ast {
	pub fn empty() -> Self {
		vecde![].into()
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
	let Some(Token::OpenBracket) = tokenstream.0.pop_front() else {
		return AstNode::Text(":".to_string() + &name);
	};

	// TODO: Attributes

	AstNode::Tag(Tag {
		name,
		attributes: HashMap::new(),
		contents: Ast::parse_until(tokenstream, Some(Token::CloseBracket)),
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{lexer::lex, vecde};

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

		let ast = Ast::parse(tokenstream.into());

		assert_eq!(
			ast,
			vecde![
				AstNode::Var(Var {
					name: "x".into(),
					contents: vecde![AstNode::Text(" X".into())].into()
				}),
				AstNode::AccessVar("x".into()),
			]
			.into()
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

		let ast = Ast::parse(tokenstream.into());

		assert_eq!(
			ast,
			vecde![
				AstNode::Tag(Tag {
					name: "a".into(),
					attributes: HashMap::new(),
					contents: Ast::empty()
				}),
				AstNode::Tag(Tag {
					name: "b".into(),
					attributes: HashMap::new(),
					contents: Ast::empty()
				}),
			]
			.into()
		);
	}

	#[test]
	fn nested_tags() {
		let tokenstream = vecde![
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
			vecde![
				AstNode::Tag(Tag {
					name: "a".into(),
					attributes: HashMap::new(),
					contents: vecde![AstNode::Tag(Tag {
						name: "b".into(),
						attributes: HashMap::new(),
						contents: Ast::empty()
					}),]
					.into()
				}),
				AstNode::Tag(Tag {
					name: "c".into(),
					attributes: HashMap::new(),
					contents: Ast::empty()
				}),
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

		let ast = Ast::parse(lex(str));

		assert_eq!(
			ast,
			vecde![
				AstNode::Var(Var {
					name: "title".into(),
					contents: vecde![AstNode::Tag(Tag {
						name: "title".into(),
						attributes: HashMap::new(),
						contents: vecde![AstNode::text("My Webpage")].into()
					})]
					.into()
				}),
				AstNode::Tag(Tag {
					name: "head".into(),
					attributes: HashMap::new(),
					contents: vecde![AstNode::AccessVar("title".into())].into()
				}),
				AstNode::Tag(Tag {
					name: "body".into(),
					attributes: HashMap::new(),
					contents: vecde![AstNode::Tag(Tag {
						name: "p".into(),
						attributes: HashMap::new(),
						contents: vecde![AstNode::text("&title")].into()
					})]
					.into()
				})
			]
			.into()
		);
	}
}
