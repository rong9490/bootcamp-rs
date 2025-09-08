use anyhow::{Ok, Result};
use clap::{Parser, command};

// 第一章, 第一小节: _01_csv_convert
use section01::clap_client::_01_csv_convert::{
    cli_command::{CsvSubCommand, csv_convert},
    utils::get_csv_output_filename,
};

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
    #[command(name = "csv", about = "CSV转换")] // 第一小节内容
    Csv(CsvSubCommand),
    // #[command(name = "gpass", about = "生成随机密码")]
    // GenPass(GenPassSubCommand),
    // #[command(subcommand)] // 二级嵌套
    // Base64(Base64Sub),
    // #[command(subcommand)] // 二级嵌套
    // Encrypt(TextEncryptSub),
}

fn main() -> Result<()> {
    println!("Section01 - 终端应用!");
    tracing_subscriber::fmt::init(); // 日志追踪

    // 命令行实例解析
    let cli: CliMajor = CliMajor::parse();
    println!("{:#?}", cli);

    let command: SubCommand = cli.command;
    println!("具体命令: {:?}", command);

    match command {
        SubCommand::Csv(csv_cmd) => {
            let CsvSubCommand {
                format,
                input,
                output,
                delimiter: _delimiter,
                skip_header: _skip_header,
            } = csv_cmd;
            let output: String = get_csv_output_filename(output, format);

            csv_convert(format, input, output)?
        }
        _ => unreachable!(),
    }

    Ok(())
}
