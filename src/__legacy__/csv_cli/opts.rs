use super::utils::verify_file_exists;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "tiger-csv-cli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand, // 嵌套二级命令, 枚举类型
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "assets/juventus.csv")]
    pub input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    #[arg(long, default_value_t = ',')] // 字符
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}
