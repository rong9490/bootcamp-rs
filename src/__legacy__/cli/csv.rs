/* "CSV" 副命令集群 */
// 命令: cargo run csv --input ./assets/juventus.csv
use super::utils::verify_file;
use crate::{process::csv_convert::major_clap_csv, CmdExector};
use clap::Parser;
use core::fmt;
use std::str::FromStr;

// 输出格式的两种枚举
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "./assets/juventus.csv")] // 文件名需要验证存在
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>, // 表示可选

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}

// 为该结构体实现trait:
impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // 处理输出文件名, 没有则用默认的(为什么不直接用"output.json".into()? 因为文件格式是动态的)
        let output: String = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };

        // 正式执行转换(serde)
        major_clap_csv(&self.input, output, self.format)
    }
}

// HACK 理解以下的方法 parse / from
// 从字符串, 格式化提取出枚举类型
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // 这里为什么字符串执行parse就可以了呢, 我们需要自定义 "parse" / "from" 这一对行为
    format.parse()
}

/* 这两个是一对的 */
impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format!")),
        }
    }
}

// 这个实现Display 可输出
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
