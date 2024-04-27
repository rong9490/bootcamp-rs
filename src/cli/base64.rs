/* "base64编码解码" 副命令集群(!! 这里是两层) */
// 命令: cargo run base64

// HACK Target 耐心写完

use crate::CmdExector;
use std::str::FromStr;
use super::utils::verify_file;
use clap::{arg, Parser};
use enum_dispatch::enum_dispatch;

// 第二层
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode")]
    Decode(Base64DecodeOpts),
}

// 第三层
// "编码" 的命令参数
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, default_value = "standard")]
    pub format: Base64Format,
}

// "解码" 的命令参数
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

// 约束格式类型
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(value: &str) -> Result<Base64Format, anyhow::Error> {
    value.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format!")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
