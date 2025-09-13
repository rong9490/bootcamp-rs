// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use section01::CmdExector;
use section01::clap_client::clienting::Opts;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let opts: Opts = Opts::parse();
    opts.cmd.execute().await?;

    Ok(())
}
