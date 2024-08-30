use clap::Parser;
use super::csv_cli::{Opts, SubCommand, process_csv};

// cargo run -- csv --input assets/juventus.csv --output output.json --delimiter ',' --header true
pub fn major() -> anyhow::Result<()> {
    println!("major开始: csv_cli");
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts);

    // 匹配副命令
    match opts.command {
        SubCommand::Csv(csv_opts) => process_csv(&csv_opts.input, &csv_opts.output)?,
    }

    Ok(())
}
