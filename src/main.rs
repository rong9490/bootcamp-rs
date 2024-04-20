use anyhow;
use bootcamp_rs::{process_csv, Opts, SubCommand};
use clap::Parser;

// cargo deny 安装最新版:
// cargo deny check
// cargo deny inti
// 数据结构不同、依赖改变
// duskdb
// cargo run -- csv --input assets/juventus.csv

// 如何处理header!
// rdr.reader();

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // 需要 std::display
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}
