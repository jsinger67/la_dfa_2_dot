extern crate clap;
extern crate parol_runtime;
extern crate tera;

mod cli_args;
mod la_dfa_2_dot_grammar;
mod la_dfa_2_dot_grammar_trait;
mod la_dfa_2_dot_parser;

use crate::la_dfa_2_dot_parser::parse;
use crate::{cli_args::CliArgs, la_dfa_2_dot_grammar::LaDfa2DotGrammar};
use anyhow::{Context, Result};
use clap::Parser;
use parol_runtime::{log::debug, Report};
use std::fs;

// To generate:
// parol -f ./la_dfa_2_dot.par -e ./la_dfa_2_dot-exp.par -p ./src/la_dfa_2_dot_parser.rs -a ./src/la_dfa_2_dot_grammar_trait.rs -t LaDfa2DotGrammar -m la_dfa_2_dot_grammar -g

struct ErrorReporter;
impl Report for ErrorReporter {}

fn main() -> Result<()> {
    env_logger::init();
    debug!("env logger started");

    let args = CliArgs::parse();
    let file_name = args.source;

    fs::create_dir_all(&args.out_folder)?;

    let input = fs::read_to_string(file_name.clone())
        .with_context(|| format!("Can't read file {}", file_name.display()))?;
    let mut la_dfa_2_dot_grammar = LaDfa2DotGrammar::new(args.out_folder);
    match parse(&input, &file_name, &mut la_dfa_2_dot_grammar) {
        Ok(_) => Ok(()),
        Err(e) => ErrorReporter::report_error(&e, file_name),
    }
}
