use crate::lexer::{Token, TokenTree};

pub fn eval(tokens: TokenTree) -> String {
	let mut html = "<html>\n\t".to_string();
	let mut tag_stack = vec![];

	for token in tokens {
		match token {
			Token::Tag(str) => {
				html.push_str(&format!("<{}", &str));
				tag_stack.push(str);
				continue;
			}
			Token::StartTag => html.push('>'),
			Token::EndTag => {
				let tag = tag_stack.pop().unwrap();
				html.pop();
				html.push_str(&format!("</{}>", tag.trim()));
			}

			Token::StartAttribute => continue,
			Token::Attribute(str) => {
				let trimmed = str.trim();
				if !trimmed.is_empty() {
					html.push_str(&format!(" {}", trimmed));
				}
				continue;
			}
			Token::EndAttribute => continue,

			Token::Text(str) => {
				let trimmed = str.trim();
				if trimmed.is_empty() {
					continue;
				}
				html.push_str(str.trim())
			}
			Token::None => continue,

			Token::Var(_) | Token::StartVar | Token::EndVar | Token::AccessVar(_) => {
				println!("error: variables should be expanded earlier");
				continue;
			}
		}

		html.push('\n');
		for _ in 0..tag_stack.len() + 1 {
			html.push('\t');
		}
	}

	html.pop();
	html.push_str("</html>\n");
	html
}
