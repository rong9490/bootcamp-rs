use clap::Parser;
use csv::Reader; // serde
use serde::{Deserialize, Serialize};
use std::path::Path;

// 引入clap的命令解析宏, 配置其字段及行为
#[derive(Debug, Parser)]
#[command(name = "tiger-cli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand, // 二级嵌套的命令结构: 是个枚举类型
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String, // 合法性的检查! value_parser

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    output: String,

    #[arg(long, default_value_t = ',')] // 字符
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

// 验证文件是否存在
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    let exists: bool = Path::new(filename).exists();
    if exists {
        Ok(filename.into())
    } else {
        Err("文件不存在!")
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Player {
    name: String,
    position: String,
    dob: String,
    nationality: String,
    kit: u8,
}

fn main() {
    // 执行: cargo run -- csv --input test.cvs
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input).unwrap();
            let records = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Player>>();
            print!("{:?}", records);
        }
    }
}
