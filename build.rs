use std::process;

use parol::{build::Builder, ParolErrorReporter};
use parol_runtime::Report;

fn main() {
    // CLI equivalent is:
    // parol -f ./la_dfa_2_dot.par -e ./la_dfa_2_dot-exp.par -p ./src/la_dfa_2_dot_parser.rs -a ./src/la_dfa_2_dot_grammar_trait.rs -t LaDfa2DotGrammar -m la_dfa_2_dot_grammar -g
    if let Err(err) = Builder::with_explicit_output_dir("src")
        .max_lookahead(7)
        .unwrap()
        .grammar_file("la_dfa_2_dot.par")
        .expanded_grammar_output_file("../la_dfa_2_dot-exp.par")
        .parser_output_file("la_dfa_2_dot_parser.rs")
        .actions_output_file("la_dfa_2_dot_grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("LaDfa2DotGrammar")
        .user_trait_module_name("la_dfa_2_dot_grammar")
        .trim_parse_tree()
        .generate_parser()
    {
        ParolErrorReporter::report_error(&err, "la_dfa_2_dot.par").unwrap_or_default();
        process::exit(1);
    }
}
