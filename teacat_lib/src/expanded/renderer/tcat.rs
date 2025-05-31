use crate::prelude::*;

pub struct TeaCatRenderer;

impl Renderer<String> for TeaCatRenderer {
	fn render(ast: ExpandedAst) -> String {
		Self.render_ast(ast)
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
			":{}{}[{}]",
			tag.name,
			self.render_attributes(tag.attributes),
			self.render_ast(tag.contents)
		)
	}

	fn render_text(&mut self, text: String) -> String {
		encode_str(text) 
	}

	fn render_attributes(&mut self, attributes: Attributes) -> String {
		let mut rendered = String::new();

		for (key, val) in attributes.0 {
			rendered.push_str(&format!("{key}:{} ", val.into_string()));
		}

		surround_curly(rendered)
	}

	fn render_array(&mut self, array: Vec<ExpandedAst>) -> String {
		let rendered = array
			.into_iter()
			.map(|ast| format!("{};", self.render_ast(ast)))
			.collect::<Vec<String>>()
			.join("");

		surround_curly(rendered)
	}
}

fn surround_curly(str: String) -> String {
	format!("{{{str}}}")
}
