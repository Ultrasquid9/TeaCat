use crate::prelude::*;

/// Renders an [ExpandedAst] into an HTML string.
pub struct HtmlRenderer;

impl Renderer<String> for HtmlRenderer {
	fn render(ast: ExpandedAst) -> String {
		let mut renderer = Self;
		format!("<!DOCTYPE html><html>{}</html>", renderer.render_ast(ast))
	}

	fn render_ast(&mut self, ast: ExpandedAst) -> String {
		let mut rendered = String::new();

		for node in ast.0 {
			rendered.push_str(&match node {
				ExpandedNode::Tag(tag) => self.render_tag(tag),
				ExpandedNode::Text(text) => self.render_text(text),
				ExpandedNode::Array(array) => self.render_array(array),
			});
		}

		rendered
	}

	fn render_tag(&mut self, tag: ExpandedTag) -> String {
		format!(
			"<{}{}>{}</{}>",
			tag.name,
			self.render_attributes(tag.attributes),
			self.render_ast(tag.contents),
			tag.name
		)
	}

	fn render_text(&mut self, text: String) -> String {
		html_escape::encode_safe(&text).into()
	}

	fn render_attributes(&mut self, attributes: Attributes) -> String {
		let mut rendered = String::new();

		for (key, val) in attributes.0 {
			rendered.push_str(&format!(" {key}={}", val.into_string()));
		}

		rendered
	}

	fn render_array(&mut self, array: Vec<ExpandedAst>) -> String {
		array
			.into_iter()
			.map(|ast| format!("<li>{}</li>", self.render_ast(ast)))
			.collect::<Vec<String>>()
			.join("")
	}
}
