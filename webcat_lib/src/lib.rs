//! # WebCat
//! This crate contains basic tools for working with WebCat files.
//!
//! ```rust
//! use webcat_lib::prelude::*;
//!
//! let webcat_str = "
//! &title := :title[My Webpage];
//!
//! :head[
//! 	&title
//! ]
//!
//! :body[
//! 	:p[Hello, World!]
//! ]
//! "
//! .to_string();
//!
//! let tokenstream = TokenStream::lex(webcat_str);
//! let ast = Ast::parse(tokenstream).unwrap();
//! let expanded = ExpandedAst::expand(ast).unwrap();
//! let html = HtmlRenderer::render(expanded);
//!
//! assert_eq!(
//! 	html,
//! 	"<!DOCTYPE html><html><head><title>My Webpage</title></head><body><p>Hello, World!</p></body></html>".to_string()
//! );
//! ```

#![allow(clippy::tabs_in_doc_comments)]

use prelude::*;

pub mod error;
pub mod expanded;
pub mod lexer;
pub mod parser;

/// Evaluates a WebCat string.
/// # Examples
/// ```
/// use webcat_lib::prelude::*;
///
/// let webcat_string = ":head[]";
///
/// assert_eq!(
/// 	eval_webcat_string::<HtmlRenderer, String>(webcat_string).unwrap(),
/// 	"<!DOCTYPE html><html><head></head></html>".to_string(),
/// );
/// ```
pub fn eval_webcat_string<Rend: Renderer<Out>, Out>(
	webcat_string: impl Into<String>,
) -> anyhow::Result<Out> {
	let tokenstream = TokenStream::lex(webcat_string);
	let ast = Ast::parse(tokenstream)?;
	let expanded = ExpandedAst::expand(ast)?;
	Ok(Rend::render(expanded))
}

/// A macro to create a [VecDeque](std::collections::VecDeque).
/// # Examples
/// ```
/// use webcat_lib::vecdeque;
///
/// assert_eq!(
/// 	vecdeque![1, 2, 3,],
/// 	std::collections::VecDeque::from([1, 2, 3,])
/// )
/// ```
#[macro_export]
macro_rules! vecdeque {
	( $( $x:expr ),* $(,)? ) => {
		std::collections::VecDeque::from(vec![ $( $x, )* ])
	};
}

pub mod prelude {
	pub use crate::error::WebCatError;
	pub use crate::eval_webcat_string;
	pub use crate::expanded::{
		ExpandedAst, ExpandedNode, ExpandedTag,
		renderer::{Renderer, html::HtmlRenderer},
	};
	pub use crate::lexer::TokenStream;
	pub use crate::parser::{Ast, Attributes};
}
