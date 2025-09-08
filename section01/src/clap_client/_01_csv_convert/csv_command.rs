// 命令行相关处理

use super::output_format::{parse_format, OutputFormat};
use super::utils::verify_file_exists;
use anyhow::{anyhow, Error, Result};
use clap::Parser;
use csv::{Reader, StringRecord};
use serde_json::Value;
use std::fs;
use std::fs::File;
use crate::clap_client::_01_csv_convert::utils::get_csv_output_filename;
use crate::clap_client::traits::CmdExector;
use super::csv_process::csv_process;

#[derive(Debug, Parser)]
pub struct CsvSubCommand {
    // 读取文件名: 默认值 + 验证方法
    #[arg(long, default_value = "assets/juventus.csv", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(long)]
    pub output: Option<String>, // 可选的输出文件名(可选 + 另一种默认值方式)

    #[arg(long, default_value = "json", value_parser = parse_format)]
    pub format: OutputFormat, // 输出的格式, 默认yaml, 字符串转为枚举项

    #[arg(long, default_value_t = ',')]
    pub delimiter: char, // 分隔符, 默认逗号

    #[arg(long, default_value_t = true)]
    pub skip_header: bool, // 是否跳过表头, 默认跳过
}

// FIXME 执行执行器Trait
impl CmdExector for CsvSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        let output: String = get_csv_output_filename(self.output, self.format);
        csv_process(self.format, &self.input, output)
    }
}

/* 正式处理转换, 暂先处理3个参数 */
pub fn _csv_convert(format: OutputFormat, input: String, output: String) -> Result<()> {
    println!("{} / {} / {}", format, input, output);
    let mut reader: Reader<File> = Reader::from_path(input)?;

    // 读取头部: 双重mut borrow, 需要clone消除错误
    // let _headers: StringRecord = reader.headers()?.clone();

    // 生成json对象, 固定大小
    let mut vec_player: Vec<Value> = Vec::with_capacity(128);

    // 迭代器+闭包写法: let vec_player: Vec<Player> = reader.deserialize().map(|record| record.unwrap()).collect::<Vec<Player>>();

    // 暂时不 reader.deserialize(), 限定了数据结构 不够通用 / StringRecord不包含header的key, 需要用迭代器生成对象
    for result in reader.records() {
        let record = result?;
        let player: Value = record.into_iter().collect();
        vec_player.push(player);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_convert_json() {
        let input: String = String::from("assets/juventus.csv");
        let output: String = String::from("assets/juventus.json");
        let format: OutputFormat = OutputFormat::Json;

        // let result: Result<(), Error> = csv_convert(format, input, output);
        // assert!(result.is_ok());
    }

    #[test]
    fn test_csv_convert_yaml() {
        let input: String = String::from("assets/juventus.csv");
        let output: String = String::from("assets/juventus.yaml");
        let format: OutputFormat = OutputFormat::Yaml;

        // let result: Result<(), Error> = csv_convert(format, input, output);
        // assert!(result.is_ok());
    }
}
