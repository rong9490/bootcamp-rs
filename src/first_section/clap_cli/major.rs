use super::{
    base64_crate::major::{
        major_clap_base64_decode, major_clap_base64_encode, Base64DecodeOpts, Base64EncodeOpts,
        Base64Sub,
    },
    csv_convert::{
        csv_cli::{major_clap_csv, CsvOpts},
        major::CsvConventSub,
    },
    gen_pass::major::{major_clap_gen_pass, GenPassSub},
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
    Csv(CsvConventSub),

    #[command(name = "gpass", about = "生成随机密码")]
    GenPass(GenPassSub),

    #[command(subcommand)]
    Base64(Base64Sub),
}

pub fn major() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    // 副命令分发
    match opts.command {
        SubCommand::Csv(csv_opts) => {
            let CsvConventSub {
                input,
                output,
                format,
                delimiter,
                header,
            } = csv_opts;
            let output: String = if let Some(output) = output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format).into() // 缺省值, format想要转字符串, 需要实现fmt::Display trait
            };
            major_clap_csv(input, output, format)?
        }

        // 不要将cli的参数直接传入, 耦合依赖得太重
        SubCommand::GenPass(random_pwd_opts) => {
            let GenPassSub {
                uppercase,
                lowercase,
                number,
                symbol,
                length,
            } = random_pwd_opts;
            major_clap_gen_pass(uppercase, lowercase, number, symbol, length)?
        }

        SubCommand::Base64(base64_sub) => match base64_sub {
            Base64Sub::Encode(encode_opts) => {
                let Base64EncodeOpts { input, format } = encode_opts;
                major_clap_base64_encode(input, format)?
            }
            Base64Sub::Decode(decode_opts) => {
                let Base64DecodeOpts { input, format } = decode_opts;
                major_clap_base64_decode(input, format)?
            }
        },
    }
    Ok(())
}
