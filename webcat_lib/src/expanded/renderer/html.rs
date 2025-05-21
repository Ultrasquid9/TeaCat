use crate::prelude::*;

pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
	fn render(ast: ExpandedAst) -> String {
		format!("<!DOCTYPE html><html>{}</html>", Self::render_ast(ast))
	}

	fn render_ast(ast: ExpandedAst) -> String {
		let mut rendered = String::new();

		for node in ast.0 {
			rendered.push_str(&match node {
				ExpandedNode::Tag(tag) => Self::render_tag(tag),
				ExpandedNode::Text(text) => text,
			});
		}

		rendered
	}

	fn render_tag(tag: ExpandedTag) -> String {
		format!(
			"<{}{}>{}</{}>",
			tag.name,
			Self::render_attributes(tag.attributes),
			Self::render_ast(tag.contents),
			tag.name
		)
	}

	fn render_attributes(attributes: Attributes) -> String {
		let mut rendered = String::new();

		for (key, val) in attributes.0 {
			rendered.push_str(&format!(" {key}={}", val.into_string()));
		}

		rendered
	}
}
