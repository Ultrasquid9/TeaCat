#![allow(clippy::tabs_in_doc_comments)]

use std::path::PathBuf;

use clap::{ArgMatches, arg, value_parser};
use webcat_lib::prelude::*;

fn main() -> anyhow::Result<()> {
	let args = args();

	let Some(file) = args.try_get_one::<PathBuf>("file")? else {
		todo!("Error Handling")
	};

	let str = std::fs::read_to_string(file)?;

	let html = eval_webcat_string::<HtmlRenderer, _>(str)?;
	println!("{html}");

	Ok(())
}

fn args() -> ArgMatches {
	clap::command!()
		.arg(arg!([file] "The file to read").value_parser(value_parser!(PathBuf)))
		.get_matches()
}
