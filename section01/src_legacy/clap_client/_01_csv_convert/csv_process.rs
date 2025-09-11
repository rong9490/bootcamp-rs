// FIXME 最佳实践解耦: cli命令行 与 exec执行内容; 中间通过Trait + Dispatch分发调用 串联

use crate::clap_client::_01_csv_convert::output_format::OutputFormat;
use anyhow::Result;
use csv::{Reader, StringRecord};
use serde_json::Value;
use std::fs::File;
use std::fs;

pub fn csv_process(format: OutputFormat, input: &str, output: String) -> Result<()> {
    println!("csv_process!!");
    println!("{} / {} / {}", format, input, output);

    let mut reader = Reader::from_path(input)?;
    let mut vec_player: Vec<Value> = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    // 迭代器+闭包写法: let vec_player: Vec<Player> = reader.deserialize().map(|record| record.unwrap()).collect::<Vec<Player>>();

    for result in reader.records() {
        // let player: Value = record.into_iter().collect();
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器 [(header, record), ..]
        // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
        let json_value: Value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        vec_player.push(json_value);
    }

    // 按格式分发处理
    let content: String = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&vec_player)?,
        OutputFormat::Yaml => serde_yaml::to_string(&vec_player)?,
    };

    // 最后, 写入文件
    fs::write(output.clone(), content)?;
    println!("文件已写入完成! {}", output);

    Ok(())
}
