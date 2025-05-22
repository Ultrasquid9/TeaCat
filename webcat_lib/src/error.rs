use std::{
	error::Error,
	fmt::{Display, Formatter, Result},
};

use anstyle::{AnsiColor, Color, Style};

use crate::lexer::Token;

const ERR: Style = colorstyle(AnsiColor::Red);
const HELP: Style = colorstyle(AnsiColor::Magenta);
const DARK: Style = colorstyle(AnsiColor::BrightBlack);
const DEFAULT: Style = colorstyle(AnsiColor::White);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
	pub number: usize,
	pub text: String,
}

#[derive(Debug, Clone)]
pub enum WebCatError {
	UndefinedVarError(String, Line),
	UnexpectedAttribute(Token, Line),
}

impl Display for Line {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.write_str(&format!(
			"{DARK}{} | {DEFAULT}{}",
			self.number + 1,
			self.text
		))
	}
}

impl Default for Line {
	fn default() -> Self {
		Self {
			number: 0,
			text: "".into(),
		}
	}
}

impl Display for WebCatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.write_str(&match self {
			Self::UndefinedVarError(var, line) => write_err(
				format!("variable '&{var}' undefined"),
				format!("to insert an '&' directly, use a backslash: '\\&{var}'"),
				line.clone(),
			),
			Self::UnexpectedAttribute(token, line) => write_err(
				format!("unexpected token in attributes: '{token}'"),
				format!("try surrounding the token with quotation marks: '\"{token}\"'"),
				line.clone(),
			),
		})
	}
}

impl Error for WebCatError {}

fn write_err(err_message: String, help_message: String, line: Line) -> String {
	format!(
		"{ERR}{err_message}{DEFAULT}\n\n{line}\n\n{DARK}-> {HELP}help: {help_message}{DEFAULT}\n"
	)
}

const fn colorstyle(color: AnsiColor) -> Style {
	Style::new().fg_color(Some(Color::Ansi(color)))
}
