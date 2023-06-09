use std::path::PathBuf;

use clap::Parser;

/// la_dfa_2_dot
//
/// takes a generated parser source and generates dot files for each generated
/// lookahead DFA inside it within the given output folder.
//
/// It also inserts the correct terminal name in lieu of the usually given terminal index into the
/// generated dot files to increase readability.
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct CliArgs {
    /// Input parser source
    #[arg(short = 'f', long = "file")]
    pub source: PathBuf,

    /// Output folder for the generated dot files
    #[arg(short = 'o', long = "out_folder")]
    pub out_folder: PathBuf,
}
