// use clap::Parser;

// #[derive(Debug, Parser)]
// #[command(name = "rcli")]
// pub struct Opts {
//     #[command(subcommand)]
//     pub cmd: SubCommand,
// }

// #[derive(Debug, Parser)]
// pub enum SubCommand {
//     #[command()]
//     Csv(CsvOpts),
// }

// #[derive(Debug, Parser)]
// pub struct CsvOpts {
//     #[arg(short, long)] // 文件名需要验证存在
//     pub input: String,

//     #[arg(short, long)] // "output.json".into()
//     pub output: Option<String>, // 表示可选吗?

//                                 // #[arg(long, value_parser = parse_format, default_value = "json")]
//                                 // pub format: OutputFormat,
// }

use super::genpass::GenPassOpts;
use clap::Parser;
use enum_dispatch::enum_dispatch;

/// 陈天Rust训练营 01_rcli
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}
