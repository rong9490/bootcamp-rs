use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{
    fmt,
    io::{stdin, Read},
    path::Path,
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

pub fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
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

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader = if input == "-" {
        Box::new(stdin()) as Box<dyn Read>
    } else {
        let file = std::fs::File::open(input)?;
        Box::new(file) as Box<dyn Read>
    };
    Ok(reader)
}

// 专注处理'编码'
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // 两种编码类型
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buffer),
    };
    println!("{}", encoded);
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    // let mut buffer = Vec::new();
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let buffer = buffer.trim(); // 去掉可能的换行符

    // 两种解码类型
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buffer)?, // clippy: 去掉引用&
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer)?,
    };
    // TODO 可能不是字符串
    let decoded_str = String::from_utf8(decoded)?;
    println!("{}", decoded_str);
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

    // #[test]
    // fn test_verify_input_file() {
    //     assert_eq!(
    //         verify_input_file("assets/juventus.csv"),
    //         Ok("assets/juventus.csv".into())
    //     );
    //     assert_eq!(verify_input_file("assets/a.csv"), Err("文件不存在!"));
    //     assert_eq!(verify_input_file("-"), Ok("-".into()));
    // }

    // #[test]
    // fn test_parse_base64_format() {
    //     // assert_eq!(parse_base64_format("standard"), Ok(Base64Format::Standard));
    //     // assert_eq!(parse_base64_format("urlsafe"), Ok(Base64Format::UrlSafe));
    //     // assert_eq!(parse_base64_format("invalid"), Err(anyhow::anyhow!("无效的base64格式: invalid")));
    // }

    #[test]
    fn test_process_encode() -> anyhow::Result<()> {
        // 测试标准编码
        let input = "测试文本";
        let result = process_encode(input, Base64Format::Standard)?;
        assert_eq!(result, "5rWL6K+V5paH5pys");

        // // 测试URL安全编码
        // let input = "Hello, World!";
        // let result = process_encode(input, Base64Format::UrlSafe)?;
        // assert_eq!(result, "SGVsbG8sIFdvcmxkIQ");

        // // 测试空输入
        // let input = "";
        // let result = process_encode(input, Base64Format::Standard)?;
        // assert_eq!(result, "");

        Ok(())
    }
}
