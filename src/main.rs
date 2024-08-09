// use clap::Parser;
// use rcli::cli::entry::Opts;
// use rcli::CmdExector; // 这里必须明确导出, 否则识别不了execute

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     tracing_subscriber::fmt::init();
//     let opts: Opts = Opts::parse();
//     println!("{:?}", opts);
//     opts.cmd.execute().await?;
//     Ok(())
// }

// rcli csv -i

use clap::Parser;

// 引入clap的命令解析宏, 配置其字段及行为
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
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
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    output: String,

    #[arg(long, default_value_t = ',')] // 字符
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    println!("Hello, rust!");
}