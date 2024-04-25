use clap::Parser;
use crate::utils::CmdExector;

#[derive(Debug, Parser)]
// (副命令)案例一: 密码生成器
// 命令: cargo run genpass --length 50
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 20)]
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

// 为GenPassOpts这个静态的结构体, 实现CmdExector这个Trait接口方法
impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        todo!()
    }
}