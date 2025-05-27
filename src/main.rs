#![allow(clippy::tabs_in_doc_comments)]

use std::{fs, path::PathBuf, process::ExitCode};

use anstyle::{AnsiColor, Color, Style};
use anyhow::{anyhow, Result as CatResult};
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

fn run() -> CatResult<()> {
	let args = args();

	let Some(file) = args.try_get_one::<PathBuf>("file")? else {
		return Err(anyhow!("no file provided"));
	};

	let str = fs::read_to_string(file)?;
	let html = eval(str)?;

	if let Some(file) = args.try_get_one::<PathBuf>("out")? {
		fs::write(file, html)?;
	} else {
		println!("{html}");
	}

	Ok(())
}

fn args() -> ArgMatches {
	clap::command!()
		.arg(arg!([file] "The file to read").value_parser(value_parser!(PathBuf)))
		.arg(
			arg!(-o --out <FILE> "The file to output to")
				.required(false)
				.value_parser(value_parser!(PathBuf)),
		)
		.get_matches()
}

fn eval(str: String) -> CatResult<String> {
	match eval_webcat_string::<HtmlRenderer, _>(&str) {
		Err(err) => Err(if let Some(fancyerr) = err.downcast_ref::<WebCatError>() {
			anyhow!(fancyerr.err_fancy(str))
		} else {
			err
		}),
		ok => ok,
	}
}

const fn colorstyle(color: AnsiColor) -> Style {
	Style::new().fg_color(Some(Color::Ansi(color)))
}
