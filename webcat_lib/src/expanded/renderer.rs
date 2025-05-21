use crate::prelude::*;

pub mod html;

/// Renders an [ExpandedAst] into another (typically human-readable) form. 
pub trait Renderer<Out> {
	fn render(ast: ExpandedAst) -> Out;
	fn render_ast(&mut self, ast: ExpandedAst) -> Out;
	fn render_tag(&mut self, tag: ExpandedTag) -> Out;
	fn render_text(&mut self, text: String) -> Out;
	fn render_attributes(&mut self, attributes: Attributes) -> Out;
}
