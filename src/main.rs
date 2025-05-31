#![allow(clippy::tabs_in_doc_comments)]

use std::{fs, path::PathBuf, process::ExitCode};

use anstyle::{AnsiColor, Color, Style};
use anyhow::{Result as CatResult, anyhow};
use cliargs::RendererArg;
use teacat_lib::prelude::*;

mod cliargs;

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
	let args = cliargs::args();

	let Some(file) = args.try_get_one::<PathBuf>("file")? else {
		return Err(anyhow!("no file provided"));
	};
	let out = args.try_get_one::<PathBuf>("out")?;

	let fun = match args.try_get_one::<RendererArg>("renderer")? {
		Some(RendererArg::TeaCat) => run::<TeaCatRenderer>,
		_ => run::<HtmlRenderer>,
	};

	if args.get_flag("stress_test") {
		for _ in 0..10000 {
			fun(file, out)?;
		}
	}

	fun(file, out)
}

fn run<R: Renderer<String>>(file: &PathBuf, out: Option<&PathBuf>) -> CatResult<()> {
	let str = fs::read_to_string(file)?;
	let html = eval::<R>(str)?;

	if let Some(file) = out {
		fs::write(file, html)?;
	} else {
		println!("{html}");
	}

	Ok(())
}

fn eval<R: Renderer<String>>(str: String) -> CatResult<String> {
	match eval_teacat_string::<R, _>(&str) {
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
