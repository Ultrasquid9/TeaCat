use eval::eval;
use lexer::lex;

pub mod eval;
pub mod lexer;

const INPUT: &str = r#"
# Comments use hashtags

# Variables
$header <- Hello, World!;

# Just about anything can be assigned to a variable
$title <- :title[
  My Webpage
];

# Macros
# Probably similar to Rust's declarative macros
# Start with a !, the rest of the syntax idk yet

:head[
	# An & symbol allows you to access a variable 
	&title;
]

:body[
	:p[
		# A backslash escapes the following character
		\&title # will print "title" in the generated HTML 
	]
]
"#;

fn main() {
	let tokens = lex(INPUT.into());
	let html = eval(tokens);
	println!("{}", html);
}
