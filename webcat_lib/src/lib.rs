//! # WebCat
//! This crate contains basic tools for working with WebCat files.
//!
//! ```rust
//! use webcat_lib::prelude::*;
//! use std::collections::HashMap;
//!
//! let webcat_str = "
//! &title := :title[My Webpage];
//!
//! :head[
//! 	&title
//! ]
//!
//! :body[
//! 	:p[
//! 		Hello, World!
//! 	]
//! ]
//! "
//! .to_string();
//!
//! let tokenstream = TokenStream::lex(webcat_str);
//! let ast = Ast::parse(tokenstream);
//! let expanded = ExpandedAst::expand(ast, &HashMap::new());
//! let html = HtmlRenderer::render(expanded);
//!
//! assert_eq!(
//! 	html,
//! 	"<!DOCTYPE html><html><head><title>My Webpage</title></head><body><p>Hello, World!</p></body></html>".to_string()
//! );
//! ```

#![allow(clippy::tabs_in_doc_comments)]

pub mod expanded;
pub mod lexer;
pub mod parser;
pub mod utils;

pub mod prelude {
	pub use crate::expanded::{
		ExpandedAst, ExpandedNode, ExpandedTag,
		renderer::{Renderer, html::HtmlRenderer},
	};
	pub use crate::lexer::TokenStream;
	pub use crate::parser::{Ast, Attributes};
}
