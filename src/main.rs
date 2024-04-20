use std::path::Path;

use clap::{Parser, Subcommand};

// 需要我们创建一个struct 承载我们的应用参数

// 最终命令: rcli csv -i input.csv -o output.json --header -d ','

#[derive(Debug, Parser)] // 属性宏, 自动展开 clap::Parser trait
#[command(name = "rcli", version, author, about, long_about = None)] // 提供元信息, cargo.toml里面取
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
    // #[arg(short)]
    // name: String,
    // name2: String,

    // #[arg(short, long)] // 配置命令行参数的元信息
    // age: Option<i8>,
}

// 目前只支持这一个subCommand, 后续可以拓展
#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "Convert CSV to json")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String, // 没有缺省值, 必传  合法性检查(文件是否存在)

    #[arg(short, long, default_value = "output.json")]
    // default_value: from trait 实现了: "output.json".into()
    output: String,

    #[arg(short, long, default_value_t = ',')] // 单引号
    delimiter: char,

    #[arg(short = 'h', long, default_value_t = true)] // default_value_t
    header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // 该路径的文件存在不
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        // Err("file not exists".into())
        Err("file not exists") // 静态版本, 字面量字符串, 进程相同static
    }
}

fn main() {
    let cli: Opts = Opts::parse();
    println!("{:?}", cli);
    // println!("name2: {}", cli.name2);
    // println!("age: {:?}", cli.age);
}
