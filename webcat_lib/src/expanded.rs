use std::collections::HashMap;

use crate::{
	error::{Line, WebCatError},
	parser::{Ast, AstNode, Attributes, Tag},
};

pub mod renderer;

/// An [Ast] that has had all variables expanded out and removed, and is ready for rendering.
#[derive(Debug, Clone)]
pub struct ExpandedAst(Vec<ExpandedNode>);

#[derive(Debug, Clone)]
pub enum ExpandedNode {
	Text(String),
	Tag(ExpandedTag),
}

#[derive(Debug, Clone)]
pub struct ExpandedTag {
	pub name: String,
	pub attributes: Attributes,
	pub contents: ExpandedAst,
}

impl ExpandedAst {
	pub fn expand(ast: Ast, vars: &HashMap<String, ExpandedAst>) -> anyhow::Result<Self> {
		let mut html = Self(vec![]);
		let mut vars = vars.clone();

		for node in ast.0 {
			match node {
				AstNode::Var(var) => {
					vars.insert(var.name, ExpandedAst::expand(var.contents, &vars)?);
				}
				AstNode::AccessVar(var) => {
					let Some(contents) = vars.get(&var) else {
						return Err(WebCatError::UndefinedVarError(
							var,
							todo!("Get line number over here")
						)
						.into());
					};
					html.0.append(&mut contents.0.clone());
				}
				AstNode::Tag(tag) => html
					.0
					.push(ExpandedNode::Tag(ExpandedTag::from_tag(tag, &vars)?)),
				AstNode::Text(text) => html.0.push(ExpandedNode::Text(text)),
			}
		}

		Ok(html)
	}
}

impl ExpandedTag {
	fn from_tag(tag: Tag, vars: &HashMap<String, ExpandedAst>) -> anyhow::Result<Self> {
		Ok(Self {
			name: tag.name,
			attributes: tag.attributes,
			contents: ExpandedAst::expand(tag.contents, vars)?,
		})
	}
}
