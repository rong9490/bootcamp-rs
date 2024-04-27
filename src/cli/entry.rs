use super::csv::CsvOpts;
use super::genpass::GenPassOpts;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use super::base64::Base64SubCommand;

/// 陈天Rust训练营 01_rcli
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

// 枚举: 同一时间只存在一种情况
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "转换csv文件")]
    Csv(CsvOpts), // "CSV读取"

    #[command(name = "genpass", about = "密码生成器")]
    GenPass(GenPassOpts), // "密码生成" 具体的内容单独维护

    // #[command(subcommand)]
    // Base64(Base64SubCommand), // 注意: 这里多嵌套了一层, CmdExector 如何处理
}
