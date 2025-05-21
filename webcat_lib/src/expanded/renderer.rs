use crate::prelude::*;

pub mod html;

pub trait Renderer {
	fn render(ast: ExpandedAst) -> String;
	fn render_ast(ast: ExpandedAst) -> String;
	fn render_tag(tag: ExpandedTag) -> String;
	fn render_attributes(attributes: Attributes) -> String;
}
