use anyhow::{Ok, Result};
use clap::{Parser, command};

use section01::clap_client::{
    // 第一章, 第一小节: _01_csv_convert
    _01_csv_convert::{
        csv_command::CsvSubCommand,
        utils::get_csv_output_filename,
    },
    // 第一章, 第二小节: _02_gen_pass
    _02_gen_pass::gpass_command::GenPassSubCommand, traits::CmdExector,
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
    #[command(name = "gpass", about = "生成随机密码")]
    GenPass(GenPassSubCommand),
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

    // 模式匹配 -> 外部处理执行, 后续升级"内部关联", 只暴露一个execute方法调用
    match command {
        SubCommand::Csv(csv_cmd) => {
            csv_cmd.execute(); // TODO async execute().await
        }
        SubCommand::GenPass(gpass_cmd) => {
            let GenPassSubCommand {
                length,
                uppercase,
                lowercase,
                number,
                symbol,
            } = gpass_cmd;
        }
        _ => unreachable!(), // internal error: entered unreachable code
    }

    Ok(())
}
