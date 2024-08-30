use clap::Parser;
use super::csv_cli::{Opts, SubCommand, process_csv};

// cargo run -- csv --input assets/juventus.csv --output output.json --delimiter ',' --header true
pub fn major() -> anyhow::Result<()> {
    println!("major开始: csv_cli");
    let opts: Opts = Opts::parse();
    // Opts { command: Csv(CsvOpts { input: "input.json", output: "output.json", delimiter: ',', header: true }) }
    println!("{:?}", opts); // 命令行输入已经准备完成

    match opts.command {
        SubCommand::Csv(csv_opts) => process_csv(&csv_opts.input, &csv_opts.output)?,
    }

    println!("major结束: csv_cli");
    Ok(())
}
