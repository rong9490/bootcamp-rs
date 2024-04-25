pub mod entry;
pub mod genpass;

// pub use self::entry::*;

// mod base64;
// mod csv;
// mod text;
// mod utils;

// use self::text::{TextSignFormat, TextSubCommand};
// // 对外暴露使用
// pub use self::{csv::*, utils::*, text::*};

// mod genpass;

// use clap::Parser; // 命令行参数
// use enum_dispatch::enum_dispatch; // 动态分发
// use std::path::{Path, PathBuf};
// // pub use self::{base}

// /* 命令行顶层入口!! */
// #[derive(Debug, Parser)]
// #[command(name = "rcli", version, author, about, long_about = None)] // 命令行元信息
// pub struct Opts {
//     #[command(subcommand)]
//     pub cmd: SubCommand,
// }

// /* 具体的二级"副指令群" */
// #[derive(Debug, Parser)]
// #[enum_dispatch(CmdExector)]
// pub enum SubCommand {
//     #[command(name = "csv", about = "use CSV")]
//     Csv(CsvOpts), // CsvOpts 具体的详细指令在单独的mod里面处理

//     #[command(subcommand)]
//     Text(TextSubCommand), // 单独处理所有的加解密
// }
