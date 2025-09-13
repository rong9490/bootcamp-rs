use clap::{Parser, Subcommand};

/* 主命令 */
#[derive(Debug, Parser)]
#[command(name = "rcli", version = "1.0.0", author = "@hejj")]
pub struct CliCommand {
    #[command(subcommand)]
    command: SubCommand,
}

/* 副命令 */
#[derive(Debug, Subcommand)]
pub enum SubCommand {
    // #[command(name = "csv", about = "CSV转换")]
    // Csv(SubCommandCsv),

    // #[command(name = "gpass", about = "生成随机密码")]
    // GenPass(GenPassSubCommand),

    // #[command(subcommand)] // 二级嵌套
    // Base64(Base64Sub),

    // #[command(subcommand)] // 二级嵌套
    // Encrypt(TextEncryptSub),
}