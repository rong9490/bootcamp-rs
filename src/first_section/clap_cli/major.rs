use super::csv_convert::csv_cli::{major_clap_csv, Opts, SubCommand};
use clap::Parser;

pub fn major() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts);

    // 匹配副命令
    match opts.command {
        SubCommand::Csv(csv_opts) => {
            let output: String = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format).into() // 缺省值, format想要转字符串, 需要实现fmt::Display trait
            };
            major_clap_csv(&csv_opts.input, output, csv_opts.format)?
        }
    }

    Ok(())
}
