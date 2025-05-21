#![allow(clippy::tabs_in_doc_comments)]

use std::{collections::HashMap, error::Error};

use webcat_lib::html::Html;
use webcat_lib::lexer::TokenStream;
use webcat_lib::parser::Ast;

pub const INPUT: &str = r#"
# Comments use hashtags

# Variables
&hello_world := Hello, World!;

# Just about anything can be assigned to a variable
&title := :title[
	My Webpage
];

# Macros
# Probably similar to Rust's declarative macros
# Start with a !, the rest of the syntax idk yet

:head[
	# An & symbol allows you to access a variable 
	&title
]

:body[
	:p[
		# A backslash escapes the following character
		\&title # will print "&title" in the generated HTML 

		:br[]

		# Use curly braces for tag attributes
		:img{
			src: "https://www.w3schools.com/images/w3schools_green.jpg", 
			alt: "Test Image",
		} []
	]
]
"#;

pub type CatResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn main() -> CatResult<()> {
	let tokenstream = TokenStream::lex(INPUT.into());
	let ast = Ast::parse(tokenstream);
	let html = Html::expand(ast, &HashMap::new());

	println!("{}", html.render());

	Ok(())
}
