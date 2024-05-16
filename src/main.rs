use crate::cli::CliOpts;
use clap::Parser;

mod cli;

fn main() {
    let _opts = CliOpts::parse();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::cli::CliOpts;

    use super::*;
    #[test]
    fn test_parse_cli() {
        let opts = CliOpts::parse_from(&["headc_test", "-n", "5", "file1", "file2"]);
        assert_eq!(opts.lines, Some(5));
        assert_eq!(opts.bytes, None);
        assert_eq!(opts.files.len(), 2);
        assert_eq!(
            opts.files,
            vec![PathBuf::from("file1"), PathBuf::from("file2")]
        );
    }
}
