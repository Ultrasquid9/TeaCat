use std::path::PathBuf;

use clap::{ArgMatches, ValueEnum, arg, builder::PossibleValue, value_parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererArg {
	Html,
	TeaCat,
}

impl ValueEnum for RendererArg {
	fn value_variants<'a>() -> &'a [Self] {
		&[Self::Html, Self::TeaCat]
	}

	fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
		Some(match self {
			Self::Html => PossibleValue::new("html"),

			Self::TeaCat => PossibleValue::new("teacat").alias("tcat").alias("tc"),
		})
	}
}

pub fn args() -> ArgMatches {
	clap::command!()
		.arg(arg!([file] "The file to read").value_parser(value_parser!(PathBuf)))
		.arg(
			arg!(-o --out <FILE> "The file to output to")
				.required(false)
				.value_parser(value_parser!(PathBuf)),
		)
		.arg(
			arg!(-r --renderer <RENDERER> "The renderer to use for the outputted file")
				.required(false)
				.value_parser(value_parser!(RendererArg)),
		)
		.arg(
			arg!(--stress_test "Runs the program several times to test performance")
				.required(false),
		)
		.get_matches()
}
