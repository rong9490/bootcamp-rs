use anyhow::{Ok, Result};

use super::csv_convert::cli_command::{csv_convert, CsvCommand};
use super::gen_pass::gen_pass_command::{deal_gen_pass, GenPassCommand};
use clap::{command, Parser};

/* cli主命令 */
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

    #[command(name = "gpass", about = "生成随机密码")]
    GenPass(GenPassCommand),
}

pub fn major() -> Result<()> {
    println!("终端应用");

    // 命令行实例解析
    let cli: CliMajor = CliMajor::parse();
    // 根据副命令分发具体的操作
    match cli.command {
        SubCommand::Csv(csv_cmd) => {
            println!("{:?}", csv_cmd);
            let CsvCommand {
                format,
                input,
                output,
                delimiter: _delimiter,
                skip_header: _skip_header,
            } = csv_cmd;
            let output: String = if let Some(output) = output {
                output.clone()
            } else {
                format!("output.{}", csv_cmd.format) // 缺省值, format想要转字符串, 需要实现fmt::Display trait
            };
            csv_convert(format, input, output)?
        }
        _ => {
            let GenPassCommand {
                length,
                uppercase,
                lowercase,
                number,
                symbol,
            } = GenPassCommand::parse();
            deal_gen_pass(uppercase, lowercase, number, symbol, length)?
        }
    }

    Ok(())
}
