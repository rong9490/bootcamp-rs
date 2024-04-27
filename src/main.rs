use clap::Parser;
use rcli::cli::entry::Opts;
use rcli::CmdExector; // 这里必须明确导出, 否则识别不了execute

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    opts.cmd.execute().await?;
    Ok(())
}
