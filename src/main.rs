// 最终命令: rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{CmdExector, Opts}; // 这里的rcli指代的是当前这整个项目

// 异步运行时
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 这个库是什么作用的 ??
    tracing_subscriber::fmt::init();

    let opts: Opts = Opts::parse();

    opts.cmd.execute().await?;
    Ok(())
}
