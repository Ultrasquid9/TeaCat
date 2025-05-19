use core::panic;
use std::collections::HashMap;

use crate::lexer::{Token, TokenTree};

pub fn variables(tokens: TokenTree) -> TokenTree {
	let mut expanded: TokenTree = vec![];
	let mut variables: HashMap<String, TokenTree> = HashMap::new();
	let mut current_var: Option<(String, TokenTree)> = None;

	for token in tokens {
		match token {
			Token::Var(name) => current_var = Some((name.clone(), vec![])),
			Token::StartVar => (),
			Token::EndVar => {
				let Some((name, var)) = current_var else {
					panic!()
				};
				current_var = None;
				variables.insert(name, var);
			}
			Token::AccessVar(name) => {
				let var = variables
					.get(&name)
					.unwrap_or_else(|| panic!("Variable {name} not yet defined"));
				expanded.append(&mut var.clone());
			}
			other => {
				if let Some((_, var)) = &mut current_var {
					var.push(other);
				} else {
					expanded.push(other);
				}
			}
		}
	}

	expanded
}
