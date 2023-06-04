use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct CliArgs {
    /// Input parser source
    #[arg(short = 'f', long = "file")]
    pub source: PathBuf,

    /// Output folder for the generated dot files
    #[arg(short = 'o', long = "out_folder")]
    pub parser: Option<PathBuf>,
}
