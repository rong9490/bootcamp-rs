use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli")]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command()]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long)] // 文件名需要验证存在
    pub input: String,

    #[arg(short, long)] // "output.json".into()
    pub output: Option<String>, // 表示可选吗?

                                // #[arg(long, value_parser = parse_format, default_value = "json")]
                                // pub format: OutputFormat,
}
