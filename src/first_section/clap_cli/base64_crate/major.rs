use std::{fmt, str::FromStr};

use clap::Parser;

use crate::first_section::clap_cli::base64_crate::utils::process_decode;

use super::utils::{base64_format_parser, process_encode, verify_input_file, Base64Format};

// 注意: 这里多了一层嵌套(三层命令 主/副/副)

// cargo run -- base64 encode --input xy123

#[derive(Debug, Parser)]
pub enum Base64Sub {
    #[command(name = "encode", about = "编码")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "解码")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(long, value_parser = verify_input_file, default_value = "-")] // -代表stdin读入的内容
    pub input: String,

    #[arg(long, value_parser = base64_format_parser, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(long)]
    pub input: String,

    #[arg(long, value_parser = base64_format_parser, default_value = "standard")]
    pub format: Base64Format,
}

pub fn major_clap_base64_encode(input: String, format: Base64Format) -> anyhow::Result<()> {
    process_encode(input, format)?;
    Ok(())
}

pub fn major_clap_base64_decode(input: String, format: Base64Format) -> anyhow::Result<()> {
    process_decode(input, format)?;
    Ok(())
}
