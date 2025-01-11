use anyhow::{Ok, Result};

use super::csv_convert::{cli_command::{CsvCommand, csv_convert}, major::major as major_csv_convert};

use clap::{command, Parser};

/* cli主命令 */
// 命令: rcli csv -i assets/juventus.csv -o assets/juventus.json

#[derive(Debug, Parser)]
#[command(name = "rcli", version = "1.0.0", author = "@hejj")]
struct CliMajor {
    #[command(subcommand)]
    command: SubCommand,
}

/* 副命令 */
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "csv转换")]
    Csv(CsvCommand),
}

// cargo run -- csv -i assets/juventus.csv -o assets/juventus.json
pub fn major() -> Result<()> {
    println!("子模块 clap_cli");

    // 命令行实例解析
    let cli: CliMajor = CliMajor::parse();

    // 根据副命令分发具体的操作
    match cli.command {
        SubCommand::Csv(csv_cmd) => {
            csv_convert(csv_cmd)?;
        }
    }

    Ok(())
}
