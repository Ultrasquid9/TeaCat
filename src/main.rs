#![allow(clippy::tabs_in_doc_comments)]

use std::{fs, path::PathBuf, process::ExitCode};

use anstyle::{AnsiColor, Color, Style};
use anyhow::{Result as CatResult, anyhow};
use clap::{ArgMatches, arg, value_parser};
use teacat_lib::prelude::*;

const ERR: Style = colorstyle(AnsiColor::Red);
const DEFAULT: Style = colorstyle(AnsiColor::White);
const BOLD: Style = Style::new().bold();

fn main() -> ExitCode {
	match teacat() {
		Ok(_) => ExitCode::SUCCESS,
		Err(e) => {
			eprintln!("\n{BOLD}{ERR}Error{DEFAULT}: {e}{BOLD:#}");
			ExitCode::FAILURE
		}
	}
}

fn teacat() -> CatResult<()> {
	let args = args();

	let Some(file) = args.try_get_one::<PathBuf>("file")? else {
		return Err(anyhow!("no file provided"));
	};
	let out = args.try_get_one::<PathBuf>("out")?;

	if args.get_flag("stress_test") {
		for _ in 0..10000 {
			run(file, out)?;
		}
	}

	run(file, out)
}

fn run(file: &PathBuf, out: Option<&PathBuf>) -> CatResult<()> {
	let str = fs::read_to_string(file)?;
	let html = eval(str)?;

	if let Some(file) = out {
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
		.arg(
			arg!(--stress_test "Runs the program several times to test performance")
				.required(false),
		)
		.get_matches()
}

fn eval(str: String) -> CatResult<String> {
	match eval_teacat_string::<HtmlRenderer, _>(&str) {
		Err(err) => Err(if let Some(fancyerr) = err.downcast_ref::<TeaCatError>() {
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
