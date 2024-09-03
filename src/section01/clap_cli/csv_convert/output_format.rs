// 输出格式的枚举及其行为

use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy,)]
pub enum OutputFormat {
    Json,
    Yaml,
}

// (类型收窄)将输入的字符串转换为枚举类型
pub fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>() // 暂不处理错误
}

// 将 '枚举类型' 转为 '字符串'
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

// parse 方法需要实现FromStr trait
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("不支持的输出格式: {}", v),
            // _ => unreachable!(),
        }
    }
}

// 实现fmt::Display trait
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", self.into())
        write!(f, "{}", Into::<&str>::into(*self)) // 声明周期, 隐式转换
    }
}

// 好难呀~~