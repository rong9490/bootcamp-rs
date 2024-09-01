// 处理命令行相关内容

use super::output_format::{parse_format, OutputFormat};
use super::utils::verify_input_file;
use clap::Parser;
use csv::{Reader, StringRecord};
use serde_json::Value;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)] // 副命令
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

    #[arg(long)]
    pub output: Option<String>, // 可选的

    // 额外添加两个内容: value_parser, from的操作
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat, // 选择的格式, 枚举

    #[arg(long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// 处理csv副命令, 流程
pub fn major_clap_csv(input: String, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;

    // 1. 迭代器 + 闭包
    // let vec_player: Vec<Player> = reader
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();
    // 2. for + push

    // 读取头部: 双重mut borrow, 需要clone消除错误
    let headers = reader.headers()?.clone();
    let mut vec_player: Vec<Value> = Vec::with_capacity(128);

    // 暂时不 reader.deserialize(), 限定了数据结构 不够通用
    // StringRecord不包含header的key, 需要用迭代器生成对象
    for record in reader.records() {
        let player: StringRecord = record?;

        let origin_player = headers.iter().zip(player.iter()).collect::<Value>();
        vec_player.push(origin_player);
    }

    // 分格式处理
    let content: String = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&vec_player)?,
        OutputFormat::Yaml => serde_yaml::to_string(&vec_player)?,
        // OutputFormat::Toml => toml::to_string(&vec_player)?,
    };

    // 最后, 写入文件
    // print!("{}", output);
    fs::write(output, content)?;

    Ok(())
}
