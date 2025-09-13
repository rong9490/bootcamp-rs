use anyhow::{Ok, Result};
use clap::{Parser, command};
// use enum_dispatch::enum_dispatch;
use section01::legacy_cli::Opts;

// use section01::clap_client::{
//     // 第一章, 第一小节: _01_csv_convert
//     _01_csv_convert::{csv_command::CsvOpts, utils::get_csv_output_filename},
//     // 第一章, 第二小节: _02_gen_pass
//     _02_gen_pass::gpass_command::GenPassSubCommand,
//     traits::CmdExector,
// };

// #[derive(Debug, Parser)]
// #[command(name = "rcli", version, author, about, long_about = None)]
// pub struct Opts {
//     #[command(subcommand)]
//     pub cmd: SubCommand,
// }

// #[derive(Debug, Parser)]
// #[enum_dispatch(CmdExector)]
// pub enum SubCommand {
//     #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
//     Csv(CsvOpts),
//     // #[command(name = "genpass", about = "Generate a random password")]
//     // GenPass(GenPassOpts),
//     // #[command(subcommand, about = "Base64 encode/decode")]
//     // Base64(Base64SubCommand),
//     // #[command(subcommand, about = "Text sign/verify")]
//     // Text(TextSubCommand),
//     // #[command(subcommand, about = "HTTP server")]
//     // Http(HttpSubCommand),
// }
// #[derive(Debug, Parser)]
// #[enum_dispatch(CmdExector)]
// pub enum SubCommand {
//     #[command(name = "csv", about = "CSV转换")] // 第一小节内容
//     Csv(CsvOpts),
//     // #[command(name = "gpass", about = "生成随机密码")]
//     // GenPass(GenPassSubCommand),
//     // #[command(subcommand)] // 二级嵌套
//     // Base64(Base64Sub),
//     // #[command(subcommand)] // 二级嵌套
//     // Encrypt(TextEncryptSub),
// }

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await?;

    // println!("Section01 - 终端应用!");
    // tracing_subscriber::fmt::init(); // 日志追踪

    // 命令行实例解析
    // let cli = Opts::parse();

    // let opts = Opts::parse();
    // opts.cmd.execute().await?;
    // cli.cmd.execute();

    // println!("{:#?}", cli);
    // let cmd: SubCommand = cli.cmd;
    // println!("具体命令: {:?}", cmd);

    // tracing_subscriber::fmt::init();
    // let opts = Opts::parse();
    // opts.cmd.execute().await?;

    // cmd.execute();

    // 模式匹配 -> 外部处理执行, 后续升级"内部关联", 只暴露一个execute方法调用
    // match command {
    //     SubCommand::Csv(csv_cmd) => {
    //         csv_cmd.execute(); // TODO async execute().await
    //     }
    //     SubCommand::GenPass(gpass_cmd) => {
    //         let GenPassSubCommand {
    //             length,
    //             uppercase,
    //             lowercase,
    //             number,
    //             symbol,
    //         } = gpass_cmd;
    //     }
    //     _ => unreachable!(), // internal error: entered unreachable code
    // }

    Ok(())
}
