use clap::Parser;
use super::csv_cli::{Opts, SubCommand, process_csv};

// cargo run -- csv --input assets/juventus.csv --output output.json --delimiter ',' --header true
// cargo run -- csv --input assets/juventus.csv
pub fn major() -> anyhow::Result<()> {
    println!("major开始: csv_cli");
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

    // 匹配副命令
    match opts.command {
        SubCommand::Csv(csv_opts) => {
            let output: String = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format).into() // 缺省值, format想要转字符串, 需要实现fmt::Display trait
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?
        }
    }

    Ok(())
}
