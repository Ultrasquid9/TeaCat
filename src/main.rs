#![allow(clippy::tabs_in_doc_comments)]

use std::{path::PathBuf, process::ExitCode};

use anstyle::{AnsiColor, Color, Style};
use clap::{ArgMatches, arg, value_parser};
use webcat_lib::prelude::*;

const ERR: Style = colorstyle(AnsiColor::Red);
const DEFAULT: Style = colorstyle(AnsiColor::White);
const BOLD: Style = Style::new().bold();

fn main() -> ExitCode {
	match run() {
		Ok(_) => ExitCode::SUCCESS,
		Err(e) => {
			eprintln!("\n{BOLD}{ERR}Error{DEFAULT}: {e}{BOLD:#}");
			ExitCode::FAILURE
		}
	}
}

fn run() -> anyhow::Result<()> {
	let args = args();

	let Some(file) = args.try_get_one::<PathBuf>("file")? else {
		return Err(anyhow::anyhow!("no file provided"));
	};

	let str = std::fs::read_to_string(file)?;
	let html = eval(str)?;

	println!("{html}");
	Ok(())
}

fn args() -> ArgMatches {
	clap::command!()
		.arg(arg!([file] "The file to read").value_parser(value_parser!(PathBuf)))
		.get_matches()
}

fn eval(str: String) -> anyhow::Result<String> {
	match eval_webcat_string::<HtmlRenderer, _>(&str) {
		Err(err) => Err(if let Some(fancyerr) = err.downcast_ref::<WebCatError>() {
			anyhow::anyhow!(format!("{}", fancyerr.err_fancy(str)))
		} else {
			err
		}),
		ok => ok,
	}
}

const fn colorstyle(color: AnsiColor) -> Style {
	Style::new().fg_color(Some(Color::Ansi(color)))
}
