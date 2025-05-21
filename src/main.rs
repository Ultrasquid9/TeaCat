#![allow(clippy::tabs_in_doc_comments)]

use webcat_lib::prelude::*;

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

fn main() -> CatResult<()> {
	let html = eval_webcat_string::<HtmlRenderer, _>(INPUT.into())?;
	println!("{html}");

	Ok(())
}
