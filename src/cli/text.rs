/* Target 当前目标把这个文件耐心写完 */

use super::verify_file;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt, str::FromStr};

/* 这个副命令群的入口 */

#[derive(Debug, Parser)]
#[enum_dispatch]
pub enum TextSubCommand {
    // 支持的命令有: sign / verify / generate 三种
    #[command(about = "Sign..")] // 默认 name = sign
    Sign(TextSignOpts),
}

// "sign" 这个命令的具体接受参数
#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-" )]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    // format 用于 ; 解析处理
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

// 格式枚举
#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(value: &str) -> Result<TextSignFormat, anyhow::Error> {
    value.parse() // <--- 需要去实现标准 parse, 文本与enum的相互转换 + Display
}

/* 这两个是一对的 */
impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format!")),
        }
    }
}

// 这个实现Display
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
