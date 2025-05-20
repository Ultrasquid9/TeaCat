use std::collections::HashMap;

use crate::parser::{Ast, AstNode, Tag};

#[derive(Debug, Clone)]
pub struct Html(Vec<HtmlNode>);

#[derive(Debug, Clone)]
pub enum HtmlNode {
	Text(String),
	Tag(HtmlTag),
}

#[derive(Debug, Clone)]
pub struct HtmlTag {
	pub name: String,
	pub attributes: HashMap<String, String>,
	pub contents: Html,
}

impl Html {
	pub fn expand(ast: Ast, vars: &HashMap<String, Html>) -> Self {
		let mut html = Self(vec![]);
		let mut vars = vars.clone();

		for node in ast.0 {
			match node {
				AstNode::Var(var) => {
					vars.insert(var.name, Html::expand(var.contents, &vars));
				}
				AstNode::AccessVar(var) => {
					let Some(contents) = vars.get(&var) else {
						todo!("Error Handling")
					};
					html.0.append(&mut contents.0.clone());
				}
				AstNode::Tag(tag) => html.0.push(HtmlNode::Tag(HtmlTag::from_tag(tag, &vars))),
				AstNode::Text(text) => html.0.push(HtmlNode::Text(text)),
			}
		}

		html
	}

	pub fn render(self) -> String {
		let mut rendered = String::new();

		for node in self.0 {
			rendered.push_str(&match node {
				HtmlNode::Tag(tag) => tag.render(),
				HtmlNode::Text(text) => text,
			});
		}

		rendered
	}
}

impl HtmlTag {
	fn from_tag(tag: Tag, vars: &HashMap<String, Html>) -> Self {
		Self {
			name: tag.name,
			attributes: tag.attributes,
			contents: Html::expand(tag.contents, vars),
		}
	}

	pub fn render(self) -> String {
		let mut rendered = format!("<{}", self.name);

		for (key, val) in self.attributes {
			rendered.push_str(&format!(" {key}=\"{val}\""));
		}

		rendered.push_str(&format!(">{}</{}>", self.contents.render(), self.name));
		rendered
	}
}
