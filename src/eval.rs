use crate::lexer::{Token, TokenTree};

pub fn eval(tokens: TokenTree) -> String {
	let mut html = "<html>".to_string();
	let mut tag_stack = vec![];

	for token in tokens {
		match token {
			Token::Tag(str) => {
				html.push_str(&format!("<{}", &str));
				tag_stack.push(str);
			}
			Token::StartTag => {
				html.push('>');
			}
			Token::EndTag => {
				let tag = tag_stack.pop().unwrap();
				html.push_str(&format!("</{}>", tag.trim()));
			}

			Token::StartAttribute => continue,
			Token::Attribute(str) => html.push_str(&format!(" {str}")),
			Token::EndAttribute => continue,

			Token::Text(str) => html.push_str(str.trim()),
			Token::None => continue,
		}
	}

	html.push_str("</html>");
	html
}
