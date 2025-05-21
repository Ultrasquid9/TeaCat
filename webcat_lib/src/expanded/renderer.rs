use crate::prelude::*;

pub mod html;

/// Renders an [ExpandedAst] into a human-readable form. 
pub trait Renderer {
	fn render(ast: ExpandedAst) -> String;
	fn render_ast(ast: ExpandedAst) -> String;
	fn render_tag(tag: ExpandedTag) -> String;
	fn render_attributes(attributes: Attributes) -> String;
}
