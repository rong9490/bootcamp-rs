/* "base64编码解码" 副命令集群(!! 这里是两层) */
// 命令: cargo run base64 encode --input fixtures/b64.txt

use super::utils::verify_file;
use crate::process::b64::process_decode;
use crate::process::b64::process_encode;
use crate::utils::get_reader;
use crate::CmdExector;
use clap::{arg, Parser};
use enum_dispatch::enum_dispatch;
use std::fmt;
use std::str::FromStr;

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

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

// "解码" 的命令参数
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
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

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

// 两个二级副命令分别都需要自己的执行器
impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let ret = process_encode(&mut reader, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let ret = process_decode(&mut reader, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}
