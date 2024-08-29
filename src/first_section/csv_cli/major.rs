use clap::Parser;
use csv::{Reader, StringRecord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)] // 子命令
    command: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "CSV转其他格式")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(long, default_value = "assets/juventus.csv", value_parser = verify_input_file)] // 做一个转换 "input.json".into()
    input: String,

    #[arg(long, default_value = "output.json")]
    output: String,

    #[arg(long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

// 自定义输入文件的检验
fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if !std::path::Path::new(file_name).exists() {
        return Err("输入文件不存在");
    }
    if !file_name.ends_with(".csv") {
        return Err("输入文件必须是CSV文件");
    }
    Ok(file_name.to_string())
}

pub fn major() {
    println!("major: csv_cli");
    let opts: Opts = Opts::parse();
    // Opts { command: Csv(CsvOpts { input: "input.json", output: "output.json", delimiter: ',', header: true }) }
    println!("{:?}", opts); // 命令行输入已经准备完成

    // 读取csv文件
    // let mut reader = Reader::from_path(opts.input)?;
    // let mut records = Vec::new();
    // for record in reader.records() {
    //     records.push(record?);
    // }
    // println!("{:?}", records);
}
