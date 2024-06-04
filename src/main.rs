use clap::{value_parser, Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Read};

use crate::cli::build_command;

mod cli;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    lines: usize,
    bytes: Option<usize>,
    files: Vec<String>,
}

fn main() {
  let r = get_args();
  match r {
    Ok(config) => {
      run(config);
    },
    Err(e) => {
      println!("{}", e);
    },
  }
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        };
    }
    Ok(())
}

fn parse_positive_int(val: &String) -> MyResult<usize> {
    //unimplemented!();
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.clone().into()),
    }
}

pub fn get_args() -> MyResult<Config> {
    //let with_name = Arg::with_name("files");
    let matches = build_command().get_matches();
    // let lines = matches.get_one::<String>("lines");

    let lines = matches
        .get_one::<String>("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .get_one("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal bytes count -- {}", e))?;

    let r = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    Ok(Config {
        files: r,
        lines: lines.unwrap(),
        bytes,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)]
mod tests {
  use super::parse_positive_int;

    #[test]
    fn test_parse_positive_int() {
        let res = parse_positive_int(&"3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 3);
        let res = parse_positive_int(&"foo".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
        let res = parse_positive_int(&"0".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "0".to_string());
    }
}
