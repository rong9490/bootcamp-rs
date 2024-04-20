use std::path::Path;

use clap::Parser;

/// 陈天, Rust训练营
#[derive(Debug, Parser)]
#[command(name = "rcli", author = "Tiger", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// 这里枚举可以平行拓展多种sub命令
/// 每一种都是一种结构体struct, 包含具体的所有操作
#[derive(Debug, Parser)]
pub enum SubCommand {
    // 注意这里的csv 副命令的名称
    #[command(name = "csv", about = "Show CSV")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)] // 也必须使用Parser
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file_path)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // 这里的short为 -h 与 --help的冲突了, 取消掉
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file_path(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
