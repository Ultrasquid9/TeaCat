use std::collections::VecDeque;

use crate::lexer::{Token, TokenStream};

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

			match token {
				Token::Andpersand => nodes.push(var(tokenstream)),

				Token::Text(text) => nodes.push(AstNode::Text(text)),

				_ => todo!(),
			}
		}

		Self(nodes.into())
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
		println!("Andpercand must be followed by an Ident!");
		todo!("Error Handling")
	};

	if let Some(Token::Walrus) = tokenstream.pop_front() {
		AstNode::Var(Var::new(name, tokenstream))
	} else {
		AstNode::AccessVar(name)
	}
}

mod tests {
	#[allow(unused)]
	use crate::parser::*;

	#[test]
	fn variables() {
		let tokenstream: TokenStream = vec![
			Token::Andpersand,
			Token::Ident("x".into()),
			Token::Walrus,
			Token::Text(" X".into()),
			Token::SemiColon,
			Token::Andpersand,
			Token::Ident("x".into()),
		].into();

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
}
