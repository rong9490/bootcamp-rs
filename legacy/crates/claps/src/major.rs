use anyhow::{Ok, Result};
use clap::{command, Parser};

use super::csv_convert::cli_command::{csv_convert, CsvSubCommand};
use super::gen_pass::gen_pass_command::{deal_gen_pass, GenPassSubCommand};
use super::base64_text::base64_command::{Base64Sub, Base64DecodeOpts, Base64EncodeOpts, major_clap_base64_decode, major_clap_base64_encode};
use super::text_encrypt::text_encrypt_command::{major_clap_text_sign, major_clap_text_verify, TextSignOpts, TextVerifyOpts, TextEncryptSub};

/* cli主命令 */
#[derive(Debug, Parser)]
#[command(name = "rcli", version = "1.0.0", author = "@hejj")]
struct CliMajor {
    #[command(subcommand)]
    command: SubCommand,
}

/* 副命令 */
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "CSV转换")]
    Csv(CsvSubCommand),

    #[command(name = "gpass", about = "生成随机密码")]
    GenPass(GenPassSubCommand),

    #[command(subcommand)] // 二级嵌套
    Base64(Base64Sub),

    #[command(subcommand)] // 二级嵌套
    Encrypt(TextEncryptSub),
}

pub fn major() -> Result<()> {
    println!("终端应用");

    // 命令行实例解析
    let cli: CliMajor = CliMajor::parse();
    // 根据副命令分发具体的操作
    match cli.command {
        SubCommand::Csv(csv_cmd) => {
            println!("{:?}", csv_cmd);
            let CsvSubCommand {
                format,
                input,
                output,
                delimiter: _delimiter,
                skip_header: _skip_header,
            } = csv_cmd;
            let output: String = if let Some(output) = output {
                output.clone()
            } else {
                format!("output.{}", csv_cmd.format) // 缺省值, format想要转字符串, 需要实现fmt::Display trait
            };
            csv_convert(format, input, output)?
        }
        SubCommand::GenPass(gen_pass_cmd) => {
            let GenPassSubCommand {
                length,
                uppercase,
                lowercase,
                number,
                symbol,
            } = gen_pass_cmd;
            let _password: String = deal_gen_pass(uppercase, lowercase, number, symbol, length)?;
        },
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
        SubCommand::Encrypt(text_sub) => match text_sub {
            TextEncryptSub::Sign(sign_opts) => {
                let TextSignOpts { input, key, format } = sign_opts;
                major_clap_text_sign(input, key, format)?
            }
            TextEncryptSub::Verify(verify_opts) => {
                let TextVerifyOpts {
                    input,
                    key,
                    sig,
                    format,
                } = verify_opts;
                major_clap_text_verify(input, key, sig, format)?
            }
        },
    }

    Ok(())
}
