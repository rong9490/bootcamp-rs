// 命令行相关处理

use super::output_format::{parse_format, OutputFormat};
use super::utils::verify_file_exists;
use clap::Parser;
use csv::{Reader, StringRecord};
use serde_json::Value;
use std::fs::File;
use std::fs;

#[derive(Debug, Parser)]
pub struct CsvCommand {
    // 默认读取文件名; 文件存在性验证
    #[arg(long, default_value = "assets/juventus.csv", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(long)]
    pub output: Option<String>, // 可选的输出文件名

    #[arg(long, default_value = "json", value_parser = parse_format)]
    pub format: OutputFormat, // 输出的格式, 默认yaml, 字符串转为枚举项

    #[arg(long, default_value_t = ',')]
    pub delimiter: char, // 分隔符, 默认逗号

    #[arg(long, default_value_t = true)]
    pub skip_header: bool, // 是否跳过表头, 默认跳过
}

/* 正式处理转换, 暂先处理3个参数 */
pub fn csv_convert(format: OutputFormat, input: String, output: String) -> anyhow::Result<()> {
    println!("输出格式: {}", format);
    println!("输入文件路径: {}", input);
    println!("输出文件路径: {}", output);
    let mut reader: Reader<File> = Reader::from_path(input)?;
    // 读取头部: 双重mut borrow, 需要clone消除错误
    let _headers: StringRecord = reader.headers()?.clone();
    // 生成json对象
    let mut vec_player: Vec<Value> = Vec::with_capacity(128);

    // 暂时不 reader.deserialize(), 限定了数据结构 不够通用
    // StringRecord不包含header的key, 需要用迭代器生成对象
    for result in reader.records() {
        let record = result?;
        let player: Value = record.into_iter().collect();
        vec_player.push(player);
    }

    // 分格式处理
    let content: String = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&vec_player)?,
        OutputFormat::Yaml => serde_yaml::to_string(&vec_player)?,
    };

    // 最后, 写入文件
    fs::write(output, content)?;

    Ok(())
}
