use super::utils::{parse_base64_format, process_encode, verify_file_exists, Base64Format};
use crate::first_section::clap_cli::base64_crate::utils::process_decode;
use clap::Parser;

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
    #[arg(long, value_parser = verify_file_exists, default_value = "-")] // -代表stdin读入的内容
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(long, value_parser = verify_file_exists, default_value = "-")] // -代表stdin读入的内容
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

pub fn major_clap_base64_encode(input: String, format: Base64Format) -> anyhow::Result<()> {
    process_encode(&input, format)?;
    Ok(())
}

pub fn major_clap_base64_decode(input: String, format: Base64Format) -> anyhow::Result<()> {
    process_decode(&input, format)?;
    Ok(())
}
