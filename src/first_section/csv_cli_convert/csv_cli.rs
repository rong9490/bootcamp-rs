// 处理命令行相关内容

use super::utils::verify_input_file;
use clap::Parser;
use csv::Reader;
use super::data::Player;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)] // 子命令
    pub command: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "CSV转其他格式")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(long, default_value = "assets/juventus.csv", value_parser = verify_input_file)]
    // 做一个转换 "assets/juventus.csv".into()
    pub input: String,

    #[arg(long, default_value = "output.json")]
    pub output: String,

    #[arg(long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// 处理csv副命令
pub fn process_csv(input: &str, _output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;

    // 1. 迭代器 + 闭包
    let vec_player: Vec<Player> = reader
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();
    // 2. for + push
    let mut vec_player2: Vec<Player> = Vec::with_capacity(128);
    for record in reader.deserialize() {
        let player: Player = record?;
        vec_player2.push(player);
    }
    let json: String = serde_json::to_string_pretty(&vec_player)?;
    let json2: String = serde_json::to_string_pretty(&vec_player2)?;

    // fs::write(opts.output, json); // => ()
    // fs::write(opts.output, json2);
    assert_eq!(json, json2); // => Result<()>
    Ok(())
}
