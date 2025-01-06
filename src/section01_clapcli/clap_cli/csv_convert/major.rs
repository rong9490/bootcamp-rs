// mod功能: 转换csv成指定格式文件(json, yaml)

use anyhow::{Result, Ok};

use clap::{command, Parser};

pub fn major() -> Result<()> {
    println!("功能模块 csv_convert");
    Ok(())
}

// csv子命令
#[derive(Debug, Parser)]
pub enum SubCommand {
  #[command(name = "csv", about = "csv转换")]
  Csv(CsvCommand),
}

#[derive(Debug, Parser)]
pub struct CsvCommand {
  #[arg(short, long)]
  input: String,
}

