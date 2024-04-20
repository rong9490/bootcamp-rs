use super::utils::verify_file;
use clap::{arg, Parser};
use enum_dispatch::enum_dispatch;

/* "副命令群" 分支入口, 包括两个具体命令内容 */
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode")]
    Decode(Base64DecodeOpts),
}

// "编码" 的命令参数
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

// "解码" 的命令参数
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}
