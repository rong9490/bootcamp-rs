use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use anyhow;

// duskdb

// 定义单位数据结构
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // 驼峰命名法? 只需要对不符合的单独rename
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String, // DateTime<Utc>
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

/// 陈天, Rust训练营
#[derive(Debug, Parser)]
#[command(name = "rcli", author = "Tiger", version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

/// 这里枚举可以平行拓展多种sub命令
/// 每一种都是一种结构体struct, 包含具体的所有操作
#[derive(Debug, Parser)]
enum SubCommand {
    // 注意这里的csv 副命令的名称
    #[command(name = "csv", about = "Show CSV")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)] // 也必须使用Parser
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file_path)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    // 这里的short为 -h 与 --help的冲突了, 取消掉
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file_path(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

// cargo run -- csv --input assets/juventus.csv

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input).unwrap();
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Player = result.unwrap();
                // println!("{:?}", record);
                ret.push(record);
            }
            // 处理成json
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
            // let records = reader
            //     .deserialize()
            //     // 函数式编程 迭代器可以Map
            //     // collect 重新组装成Vec
            //     // 避免使用unwrap
            //     .map(|record| record.unwrap())
            //     .collect::<Vec<Player>>();
        }
    }

    Ok(())
}
