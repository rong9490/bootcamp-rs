use super::{
    csv_convert::{csv_cli::major_clap_csv, major::CsvConvertOpts},
    gen_pass::major::{major_clap_gen_pass, GenPassOpts},
};
use clap::Parser;

// Cli 主命令
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)] // 副命令
    pub command: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "CSV转其他格式")]
    Csv(CsvConvertOpts),

    #[command(name = "gpass", about = "生成随机密码")]
    RandomPwd(GenPassOpts),
}

pub fn major() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts);

    // 匹配副命令
    match opts.command {
        SubCommand::Csv(csv_opts) => {
            let output: String = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format).into() // 缺省值, format想要转字符串, 需要实现fmt::Display trait
            };
            major_clap_csv(&csv_opts.input, output, csv_opts.format)?
        }

        // 不要将cli的参数直接传入, 耦合依赖得太重
        SubCommand::RandomPwd(random_pwd_opts) => {
            let GenPassOpts {
                uppercase,
                lowercase,
                number,
                symbol,
                length,
            } = random_pwd_opts;
            major_clap_gen_pass(uppercase, lowercase, number, symbol, length)?
        }
    }
    Ok(())
}
