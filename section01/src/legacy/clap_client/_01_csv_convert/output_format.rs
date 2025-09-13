/* 格式化, 涉及到: From Trait 和 Into Trait */

use std::{fmt, str::FromStr};

// 签名: pub trait FromStr: Sized {}
// 输出文件格式枚举 + 及其方法
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

// 类型收窄: 将字符串转为OutputFormat, 不满足则抛错
pub fn parse_format(format_str: &str) -> Result<OutputFormat, anyhow::Error> {
    match format_str.trim() {
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        _ => Err(anyhow::anyhow!("Invalid format type: {}", format_str)), // TODO 后续全局自定义错误枚举
    }
}

/* === FromStr Trait === */

// 从 "枚举项" --> 字符串, 实现后可以调用to_string()方法
impl From<OutputFormat> for &'static str {
    fn from(format_str: OutputFormat) -> Self {
        match format_str {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

/* === Into Trait === */

// 从 "字符串" --> 枚举项
impl FromStr for OutputFormat {
    type Err = anyhow::Error; // 上下文错误类型

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        parse_format(str)
    }
}

/* === Display Trait === */

// 实现打印
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 两行代码有什么差别?
        // write!(f, "{}", self.into())
        write!(f, "{}", Into::<&str>::into(*self)) // 声明周期, 隐式转换
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_format() {
        assert!(matches!(parse_format("json").unwrap(), OutputFormat::Json));
        assert!(matches!(parse_format("yaml").unwrap(), OutputFormat::Yaml));
        assert!(parse_format("invalid").is_err());
    }

    #[test]
    fn test_from_str() {
        // 小写
        assert!(matches!(
            "json".parse::<OutputFormat>().unwrap(),
            OutputFormat::Json
        ));
        assert!(matches!(
            "yaml".parse::<OutputFormat>().unwrap(),
            OutputFormat::Yaml
        ));
        // 大写
        assert!(matches!(
            "JSON".parse::<OutputFormat>().unwrap(),
            OutputFormat::Json
        ));
        assert!(matches!(
            "YAML".parse::<OutputFormat>().unwrap(),
            OutputFormat::Yaml
        ));
    }

    #[test]
    fn test_display() {
        // to_string 调用 fmt::Display::fmt
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Yaml.to_string(), "yaml");
    }

    #[test]
    fn test_into_str() {
        // 调用 Into::<&str>::into 隐式转换
        let json_str: &str = Into::<&str>::into(OutputFormat::Json);
        let yaml_str: &str = Into::<&str>::into(OutputFormat::Yaml);
        assert_eq!(json_str, "json");
        assert_eq!(yaml_str, "yaml");
    }
}
