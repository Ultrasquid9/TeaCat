use crate::prelude::*;

pub mod html;
pub mod tcat;

/// Renders an [ExpandedAst] into another (typically human-readable) form.
pub trait Renderer<Out> {
	fn render(ast: ExpandedAst) -> Out;
	fn render_ast(&mut self, ast: ExpandedAst) -> Out;
	fn render_tag(&mut self, tag: ExpandedTag) -> Out;
	fn render_text(&mut self, text: String) -> Out;
	fn render_attributes(&mut self, attributes: Attributes) -> Out;
	fn render_array(&mut self, array: Vec<ExpandedAst>) -> Out;
}
