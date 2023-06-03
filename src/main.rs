extern crate parol_runtime;
extern crate tera;

mod la_dfa_2_dot_grammar;
// The output is version controlled
mod la_dfa_2_dot_grammar_trait;
mod la_dfa_2_dot_parser;

use crate::la_dfa_2_dot_grammar::LaDfa2DotGrammar;
use crate::la_dfa_2_dot_parser::parse;
use anyhow::{anyhow, Context, Result};
use parol_runtime::{log::debug, Report};
use std::{env, fs};

// To generate:
// parol -f ./la_dfa_2_dot.par -e ./la_dfa_2_dot-exp.par -p ./src/la_dfa_2_dot_parser.rs -a ./src/la_dfa_2_dot_grammar_trait.rs -t LaDfa2DotGrammar -m la_dfa_2_dot_grammar -g

struct ErrorReporter;
impl Report for ErrorReporter {}

fn main() -> Result<()> {
    env_logger::init();
    debug!("env logger started");

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file_name = args[1].clone();
        let input = fs::read_to_string(file_name.clone())
            .with_context(|| format!("Can't read file {}", file_name))?;
        let mut la_dfa_2_dot_grammar = LaDfa2DotGrammar::new();
        match parse(&input, &file_name, &mut la_dfa_2_dot_grammar) {
            Ok(_) => Ok(()),
            Err(e) => ErrorReporter::report_error(&e, file_name),
        }
    } else {
        Err(anyhow!("Please provide a file name as first parameter!"))
    }
}
