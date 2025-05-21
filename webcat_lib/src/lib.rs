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

use std::{collections::HashMap, error::Error};

use prelude::*;

pub mod expanded;
pub mod lexer;
pub mod parser;

/// A wrapper around [Result] containing a dynamic [Error] type.
// Meow 
pub type CatResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Evaluates a WebCat string.
/// # Examples
/// ```
/// use webcat_lib::prelude::*;
///
/// let webcat_string = ":head[]".to_string();
///
/// assert_eq!(
/// 	eval_webcat_string::<HtmlRenderer>(webcat_string).unwrap(),
/// 	"<!DOCTYPE html><html><head></head></html>".to_string(),
/// );
/// ```
pub fn eval_webcat_string<Rend: Renderer>(webcat_string: String) -> CatResult<String> {
	let tokenstream = TokenStream::lex(webcat_string);
	let ast = Ast::parse(tokenstream);
	let expanded = ExpandedAst::expand(ast, &HashMap::new());
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
	pub use crate::expanded::{
		ExpandedAst, ExpandedNode, ExpandedTag,
		renderer::{Renderer, html::HtmlRenderer},
	};
	pub use crate::lexer::TokenStream;
	pub use crate::parser::{Ast, Attributes};
	pub use crate::{CatResult, eval_webcat_string};
}
