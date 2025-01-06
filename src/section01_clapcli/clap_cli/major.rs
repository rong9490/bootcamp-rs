use anyhow::{Ok, Result};

use super::csv_convert::major::{major as major_csv_convert, SubCommand};

use clap::{command, Parser};

/* === cli主命令 === */

// 命令: rcli csv -i assets/juventus.csv -o assets/juventus.json

#[derive(Debug, Parser)]
#[command(name = "rcli", version = "1.0.0", author = "@hejj")]
struct CliMajor {
    #[command(subcommand)]
    cmd: SubCommand,
}

pub fn major() -> Result<()> {
    println!("子模块 clap_cli");
    major_csv_convert()?;
    Ok(())
}
