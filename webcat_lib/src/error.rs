use std::{
	error::Error,
	fmt::{Display, Formatter, Result},
};

use anstyle::{AnsiColor, Color, Style};

use crate::lexer::Token;
use lines::Lines;

pub mod lines;

const VERT: char = '│';
const VERT_DASH: char = '┆';

const HELP: Style = colorstyle(AnsiColor::Magenta);
const DARK: Style = colorstyle(AnsiColor::BrightBlack);
const DEFAULT: Style = colorstyle(AnsiColor::White);
const BOLD: Style = Style::new().bold();

#[derive(Debug, Clone)]
pub enum WebCatError {
	UndefinedVar(usize, String),
	UndefinedMacr(usize, String),
	UnexpectedAttr(usize, Token),
	UnexpectedToken(usize, Token),
	ExpectedIdent(usize, Token),
	EarlyEof(usize, Token),
}

impl Display for WebCatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.write_str(&match self {
			Self::UndefinedVar(_, var) => format!("variable '&{var}' undefined"),
			Self::UndefinedMacr(_, macr) => format!("macro '&{macr}' undefined"),
			Self::UnexpectedAttr(_, token) => {
				format!("unexpected input in attributes: '{token}'")
			}
			Self::UnexpectedToken(_, token) => {
				format!("unexpected token: '{token}'")
			}
			Self::ExpectedIdent(_, token) => format!("expected identifier, found '{token}'"),
			Self::EarlyEof(_, token) => format!("early end of file while seeking token '{token}'"),
		})
	}
}

impl Error for WebCatError {}

impl WebCatError {
	fn line_num(&self) -> usize {
		match self {
			Self::EarlyEof(line, _)
			| Self::UndefinedVar(line, _)
			| Self::UndefinedMacr(line, _)
			| Self::UnexpectedToken(line, _)
			| Self::UnexpectedAttr(line, _)
			| Self::ExpectedIdent(line, _) => *line,
		}
	}

	pub fn help_msg(&self) -> String {
		match self {
			Self::UndefinedVar(_, var) => {
				format!("to insert an '&' directly, use a backslash: '\\&{var}'")
			}
			Self::UndefinedMacr(_, macr) => {
				format!("to insert an '@' directly, use a backslash: '\\@{macr}'")
			}
			Self::UnexpectedAttr(_, token) => {
				format!("try surrounding the token with quotation marks: '\"{token}\"'")
			}
			Self::UnexpectedToken(_, token) => format!("remove the '{token}'"),
			Self::ExpectedIdent(_, token) => {
				format!(
					"if you intended '{token}' to be an identifier, be sure to remove or escape whitespace"
				)
			}
			Self::EarlyEof(_, token) => {
				format!("add the expected token to the end of the file: '{token}'")
			}
		}
	}

	pub fn err_fancy(&self, webcat_str: impl Into<String>) -> String {
		let help = format!(
			"{DARK}    ╰─▶ {HELP}{BOLD}help: {}{BOLD:#}{DEFAULT}",
			self.help_msg()
		);
		let lines = Lines::new(self.line_num(), webcat_str);

		format!("{BOLD}{self}{BOLD:#}\n\n{lines}{DARK}    {VERT_DASH}\n{help}\n")
	}
}

const fn colorstyle(color: AnsiColor) -> Style {
	Style::new().fg_color(Some(Color::Ansi(color)))
}
