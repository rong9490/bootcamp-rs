// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use section01::clap_client::clienting::Opts;
use section01::clap_client::utils::CmdExector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("example - csv!");
    tracing_subscriber::fmt::init();

    let opts: Opts = Opts::parse();
    opts.cmd.execute().await?;

    Ok(())
}
