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

use clap::Parser;

use super::genpass::GenPassOpts;

/// 陈天Rust训练营 01_rcli
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// 命令行总入口, 分为二级副命令(枚举, 同一时间只能收敛成某一种情况)
pub enum Opts {
    #[command(name = "genpass", about = "案例一: 密码生成器")]
    Genpass(GenPassOpts),
}
