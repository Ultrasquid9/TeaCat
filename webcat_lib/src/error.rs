use std::{
	error::Error,
	fmt::{Display, Formatter, Result},
};

use anstyle::{AnsiColor, Color, Style};

#[derive(Debug, Clone)]
pub struct Line {
	pub number: usize,
	pub text: String,
}

#[derive(Debug, Clone)]
pub enum WebCatError {
	UndefinedVarError(String, Line),
}

impl Display for WebCatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.write_str(&match self {
			Self::UndefinedVarError(var, line) => write_err(
				format!("variable '&{var}' undefined"),
				format!("to insert an '&' directly, use a backslash: '\\&{var}'"),
				line.clone(),
			),
		})
	}
}

impl Error for WebCatError {}

fn write_err(err_message: String, help_message: String, line: Line) -> String {
	const ERR: Style = colorstyle(AnsiColor::Red);
	const HELP: Style = colorstyle(AnsiColor::Magenta);
	const DARK: Style = colorstyle(AnsiColor::BrightBlack);
	const DEFAULT: Style = colorstyle(AnsiColor::White);

	// This is a mess, yet simutaneously was the EASIEST way to represent this 
	format!(
		"{}\n\n{}\n\n{}\n",
		format!("{ERR}{err_message}{DEFAULT}"),
		format!("{DARK}{} | {DEFAULT}{}", line.number, line.text),
		format!("{DARK}-> {HELP}help: {help_message}{DEFAULT}")
	)
}

const fn colorstyle(color: AnsiColor) -> Style {
	Style::new().fg_color(Some(Color::Ansi(color)))
}
