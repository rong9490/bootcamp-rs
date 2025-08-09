use clap::{Parser, command};
use toolbox::{demo::verify_file_exists};
use toolbox::format::{parse_format, OutputFormat};
use section01::client_command::client::CliCommand;

// 主命令
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

// 二级命令
#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "CSV转换")]
    Csv(CsvSubCommand),
}

// 二级命令的具体参数
#[derive(Debug, Parser)]
pub struct CsvSubCommand {
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

// rcli csv -i input.csv -o output.json
fn main() {
    println!("Hello, Section01!");
    // println!("{:#?}", CliCommand {});
}
