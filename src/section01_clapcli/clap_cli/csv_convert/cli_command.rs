// 命令行相关处理

use super::output_format::{parse_format, OutputFormat};
use super::utils::verify_file_exists;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvCommand {
    // 默认读取文件名; 文件存在性验证
    #[arg(short, long, default_value = "assets/juventus.csv", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(long)]
    pub output: Option<String>, // 可选的输出文件名

    #[arg(long, default_value = "yaml", value_parser = parse_format)]
    pub format: OutputFormat, // 输出的格式, 默认yaml, 字符串转为枚举项

    #[arg(long, default_value_t = ',')]
    pub delimiter: char, // 分隔符, 默认逗号

    #[arg(long, default_value_t = true)]
    pub skip_header: bool, // 是否跳过表头, 默认跳过
}

pub fn csv_convert(csv_cmd: CsvCommand) -> anyhow::Result<()> {
    println!("副命令: csv");
    println!("输入文件路径: {}", csv_cmd.input);
    println!(
        "输出文件路径: {}",
        // 如果未指定 "unwrap_or"
        csv_cmd.output.unwrap_or("未指定".to_string())
    );
    println!("输出格式: {}", csv_cmd.format);
    println!("分隔符: {}", csv_cmd.delimiter);
    println!("跳过表头: {}", csv_cmd.skip_header);
    Ok(())
}
