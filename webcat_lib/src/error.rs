use std::{
	error::Error,
	fmt::{Display, Formatter, Result},
};

use anstyle::{AnsiColor, Color, Style};

use crate::lexer::Token;
use lines::Lines;

pub mod lines;

const VERT: char = '│';
const VERT_DASH: char = '┊';

const HELP: Style = colorstyle(AnsiColor::Magenta);
const DARK: Style = colorstyle(AnsiColor::BrightBlack);
const DEFAULT: Style = colorstyle(AnsiColor::White);

#[derive(Debug, Clone)]
pub enum WebCatError {
	UndefinedVarError(usize, String),
	UnexpectedAttr(usize, Token),
	EarlyEof(usize, Token),
}

impl Display for WebCatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.write_str(&match self {
			Self::UndefinedVarError(_, var) => format!("variable '&{var}' undefined"),
			Self::UnexpectedAttr(_, token) => {
				format!("unexpected token in attributes: '{token}'")
			}
			Self::EarlyEof(_, token) => format!("early end of file while seeking token '{token}'"),
		})
	}
}

impl Error for WebCatError {}

impl WebCatError {
	fn line_num(&self) -> usize {
		match self {
			Self::EarlyEof(line, _)
			| Self::UndefinedVarError(line, _)
			| Self::UnexpectedAttr(line, _) => *line,
		}
	}

	pub fn help_msg(&self) -> String {
		match self {
			Self::UndefinedVarError(_, var) => {
				format!("to insert an '&' directly, use a backslash: '\\&{var}'")
			}
			Self::UnexpectedAttr(_, token) => {
				format!("try surrounding the token with quotation marks: '\"{token}\"'")
			}
			Self::EarlyEof(_, token) => {
				format!("add the expected token to the end of the file: '{token}'")
			}
		}
	}

	pub fn err_fancy(&self, webcat_str: impl Into<String>) -> String {
		let help = format!("{DARK}    ╰─▶ {HELP}help: {}{DEFAULT}", self.help_msg());
		let lines = Lines::new(self.line_num(), webcat_str);

		format!("{self}\n\n{lines}{DARK}    {VERT_DASH}\n{help}\n")
	}
}

const fn colorstyle(color: AnsiColor) -> Style {
	Style::new().fg_color(Some(Color::Ansi(color)))
}
