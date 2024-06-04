use clap::{value_parser, Arg, ArgAction, Command};

pub fn build_command() -> Command {
    Command::new("headc")
        .version("0.1.0")
        .author("Sae Nishimura")
        .about("Rust head")
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .num_args(1)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files(s)")
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .default_value("-"),
        )
}
