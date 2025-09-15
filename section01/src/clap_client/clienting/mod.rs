// 专注处理命令行相关的内容!

pub mod csv;
pub mod genpass;
// pub mod base64;

use self::csv::CsvOpts;
use self::genpass::GenPassOpts;
use super::utils::CmdExector;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

// 主命令
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
