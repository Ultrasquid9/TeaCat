//! # TeaCat
//! This crate contains basic tools for working with TeaCat files.
//!
//! ```rust
//! # fn hidden() -> anyhow::Result<()> {
//! use teacat_lib::prelude::*;
//!
//! let teacat_str = "
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
//! let tokenstream = TokenStream::lex(teacat_str);
//! let ast = Ast::parse(tokenstream)?;
//! let expanded = ExpandedAst::expand(ast)?;
//! let html = HtmlRenderer::render(expanded);
//!
//! assert_eq!(
//! 	html,
//! 	"<!DOCTYPE html><html><head><title>My Webpage</title></head><body><p>Hello, World!</p></body></html>".to_string()
//! );
//! # Ok(())
//! # }
//! # hidden().unwrap();
//! ```

#![allow(clippy::tabs_in_doc_comments)]

use prelude::*;

pub mod error;
pub mod expanded;
pub mod lexer;
pub mod parser;

/// Evaluates a TeaCat string.
/// # Examples
/// ```
/// use teacat_lib::prelude::*;
///
/// let teacat_string = ":head[]";
///
/// assert_eq!(
/// 	eval_teacat_string::<HtmlRenderer, String>(teacat_string).unwrap(),
/// 	"<!DOCTYPE html><html><head></head></html>".to_string(),
/// );
/// ```
pub fn eval_teacat_string<Rend: Renderer<Out>, Out>(
	teacat_string: impl AsRef<str>,
) -> CatResult<Out> {
	let tokenstream = TokenStream::lex(teacat_string);
	let ast = Ast::parse(tokenstream)?;
	let expanded = ExpandedAst::expand(ast)?;
	Ok(Rend::render(expanded))
}

/// Encodes a string so that it can be safely used in a TeaCat file.
/// # Examples
/// ```
/// use teacat_lib::prelude::*;
///
/// let teacat_string = ":head[]";
///
/// assert_eq!(
/// 	encode_str(teacat_string),
/// 	"\\:head\\[\\]".to_string(),
/// );
/// ```
pub fn encode_str(str: impl Into<String>) -> String {
	#[rustfmt::skip]
	const TOKENS: &[&str] = &[
		"\\", 
		"[", 
		"]", 
		"{", 
		"}", 
		"macr", 
		":=", 
		":", 
		";", 
		"&", 
		"@",
	];

	let mut str = str.into();

	for token in TOKENS {
		str = str
			.split(token)
			.collect::<Vec<&str>>()
			.join(&("\\".to_owned() + token));
	}

	str
}

/// A macro to create a [VecDeque](std::collections::VecDeque).
/// # Examples
/// ```
/// use teacat_lib::vecdeque;
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
	pub use crate::error::TeaCatError;
	pub use crate::expanded::{
		ExpandedAst, ExpandedNode, ExpandedTag,
		renderer::{Renderer, html::HtmlRenderer, tcat::TeaCatRenderer},
	};
	pub use crate::lexer::TokenStream;
	pub use crate::parser::{Ast, Attributes};
	pub use crate::{encode_str, eval_teacat_string};

	// Meow
	pub(crate) use anyhow::Result as CatResult;
}
