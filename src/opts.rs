use anyhow;
use clap::Parser;
use core::fmt;
use std::{format, path::Path, str::FromStr};

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

    #[command(name = "genpass")] // 输入 genpass 进入子分支
    GenPass(GenPassOpts),
}

// 输出的格式枚举: 暂时支持3中格式
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)] // 也必须使用Parser
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file_path)]
    pub input: String,

    #[arg(short, long)] // default_value = "output.json"
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat, // 转换成的输出数据格式, 默认是json

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // 这里的short为 -h 与 --help的冲突了, 取消掉
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_input_file_path(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

// 对输入的格式进行匹配校验
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // parse 自动调用from_str
    format.parse::<OutputFormat>()
    // match format.to_lowercase().as_str() {
    //     "json" => Ok(OutputFormat::Json),
    //     "yaml" => Ok(OutputFormat::Yaml),
    //     "toml" => Ok(OutputFormat::Toml),
    //     _ => Err("Invalid format"),
    // }
}

// from 与 into 是一对的 互为相反

// TODO 理解这行代码是什么意思
impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

// TODO 理解这行代码是什么意思
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format!")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self)) // 这句是什么意思???
    }
}

// Path: src/process.rs
