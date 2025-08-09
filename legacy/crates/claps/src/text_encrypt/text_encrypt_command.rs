use std::{fmt, str::FromStr};
use super::utils::{verify_file_exists};
use clap::Parser;

// 注意: 这里多了一层嵌套(三层命令 主/副/副)

// cargo run -- base64 encode --input xy123

#[derive(Debug, Parser)]
pub enum TextEncryptSub {
    #[command(name = "sign", about = "签名")]
    Sign(TextSignOpts),

    #[command(name = "verify", about = "验证")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = verify_file_exists)]
    pub key: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = verify_file_exists)]
    pub key: String,

    #[arg(long)]
    pub sig: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Sha256,
}

// 照旧 三件套
pub fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "sha256" => Ok(TextSignFormat::Sha256),
            _ => Err(anyhow::anyhow!("无效的签名格式: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Sha256 => "sha256",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn major_clap_text_sign(input: String, key: String, format: TextSignFormat) -> anyhow::Result<()> {
    // process_encode(&input, format)?;
    Ok(())
}

pub fn major_clap_text_verify(input: String, key: String, sig: String, format: TextSignFormat) -> anyhow::Result<()> {
    // process_decode(&input, format)?;
    Ok(())
}
