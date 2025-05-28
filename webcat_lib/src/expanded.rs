use std::collections::HashMap;

use crate::{
	parser::{AstNode, Tag},
	prelude::*,
};

pub mod renderer;

/// An [Ast] that has had all variables/macros expanded out and removed, and is ready for rendering.
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
	/// Expands an [Ast], removing all variables/macros.
	pub fn expand(ast: Ast) -> CatResult<Self> {
		Self::expand_inner(ast, &HashMap::new(), &HashMap::new())
	}

	fn expand_inner(
		ast: Ast,
		vars: &HashMap<String, ExpandedAst>,
		macrs: &HashMap<String, Ast>,
	) -> CatResult<Self> {
		let mut expanded = Self(vec![]);
		let mut vars = vars.clone();
		let mut macrs = macrs.clone();

		for node in ast.0 {
			match node {
				AstNode::Var(var) => {
					vars.insert(
						var.name,
						ExpandedAst::expand_inner(var.contents, &vars, &macrs)?,
					);
				}
				AstNode::AccessVar(line, var) => {
					let Some(contents) = vars.get(&var) else {
						return Err(WebCatError::UndefinedVar(line, var).into());
					};
					expanded.0.append(&mut contents.0.clone());
				}

				AstNode::AccessMacr(line, args, name) => {
					let mut macr_vars = HashMap::new();

					for arg in args {
						macr_vars.insert(
							arg.name,
							ExpandedAst::expand_inner(arg.contents, &vars, &macrs)?,
						);
					}

					let Some(macr) = macrs.get(&name) else {
						return Err(WebCatError::UndefinedMacr(line, name).into());
					};

					let mut expanded_macr =
						ExpandedAst::expand_inner(macr.clone(), &macr_vars, &macrs)?;
					expanded.0.append(&mut expanded_macr.0);
				}
				AstNode::Macr(macr) => {
					macrs.insert(macr.name, macr.contents);
				}

				AstNode::Tag(tag) => expanded.0.push(ExpandedNode::Tag(ExpandedTag::from_tag(
					tag, &vars, &macrs,
				)?)),
				AstNode::Text(text) => expanded.0.push(ExpandedNode::Text(text)),
			}
		}

		Ok(expanded)
	}
}

impl ExpandedTag {
	fn from_tag(
		tag: Tag,
		vars: &HashMap<String, ExpandedAst>,
		macrs: &HashMap<String, Ast>,
	) -> CatResult<Self> {
		Ok(Self {
			name: tag.name,
			attributes: tag.attributes,
			contents: ExpandedAst::expand_inner(tag.contents, vars, macrs)?,
		})
	}
}
