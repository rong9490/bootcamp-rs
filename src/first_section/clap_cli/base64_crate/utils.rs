use std::{
    fmt,
    io::{stdin, Read},
    path::Path,
    str::FromStr,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use base64::engine::general_purpose::URL_SAFE;

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

pub fn base64_format_parser(s: &str) -> Result<Base64Format, String> {
    match s {
        "standard" => Ok(Base64Format::Standard),
        "urlsafe" => Ok(Base64Format::UrlSafe),
        _ => Err(format!("无效的base64格式: {}", s)),
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("无效的base64格式: {}", s)),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// 专注处理'编码'
pub fn process_encode(input: String, format: Base64Format) -> anyhow::Result<()> {
    // HACK 是否能返回不同的类型 --> Box封装一层(提升到dyn)
    // 文件读取流程, 可以提取成公共的读取方法
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin()) as Box<dyn Read>
    } else {
        let file = std::fs::File::open(input)?;
        Box::new(file) as Box<dyn Read>
    };

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // 两种编码类型
    let encoded = match format {
        Base64Format::Standard => URL_SAFE.encode(buffer),
        Base64Format::UrlSafe => URL_SAFE.encode(buffer),
    };
    println!("encoded: {}", encoded);

    Ok(())
}

pub fn process_decode(input: String, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin()) as Box<dyn Read>
    } else {
        let file = std::fs::File::open(input)?;
        Box::new(file) as Box<dyn Read>
    };

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // 两种解码类型
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buffer)?,
        Base64Format::UrlSafe => URL_SAFE.decode(&buffer)?,
    };
    // TODO 可能不是字符串
    let decoded_str = String::from_utf8(decoded)?;
    println!("decoded: {}", decoded_str);

    Ok(())
}

// 验证文件是否存在
pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // 特殊缺省情况
    if filename == "-" {
        return Ok(filename.into());
    }

    let exists: bool = Path::new(filename).exists();
    if exists {
        Ok(filename.into())
    } else {
        Err("文件不存在!")
    }
}

// cargo test -- test_verify_input_file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(
            verify_input_file("assets/juventus.csv"),
            Ok("assets/juventus.csv".into())
        );
        assert_eq!(verify_input_file("assets/a.csv"), Err("文件不存在!"));
        assert_eq!(verify_input_file("-"), Ok("-".into()));
    }
}
