use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliOpts {
    #[arg(short = 'n',long, default_value = "10")]
    pub lines: Option<u8>,

    #[arg(short = 'c', long)]
    pub bytes: Option<u8>,

    #[arg(value_name = "FILE")]
    pub files: Vec<PathBuf>,
}
