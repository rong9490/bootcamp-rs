// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use section01::{CmdExector, Opts};
// use section01::clap_client::main_clap_client::main_clap_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await?;

    // main_clap_client();

    Ok(())
}
