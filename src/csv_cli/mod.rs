use clap::Parser;
use csv::Reader;
use data::Player;
use opts::{Opts, SubCommand};

pub mod data;
pub mod opts;
pub mod utils;

// 执行: cargo run -- csv --input assets/juventus.csv
pub fn usage() -> anyhow::Result<()> {
    print!("usage csv_cli...");

    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;

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

            // fs::write(opts.output, json);
            // fs::write(opts.output, json2);
            assert_eq!(json, json2);
        }
    }
    Ok(())
}
