pub mod env;

use anyhow::Result;
use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use std::{fs::File, io::Read};

// use crate::clap_client::clienting::base64::Base64Format;

// Box<dyn Read> Trait对象动态分发
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin()) // 标准输入 stdin
    } else {
        Box::new(File::open(input)?) // 文件读取 File::open
    };
    Ok(reader)
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

// 专注处理'编码'
// pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
//     let mut reader: Box<dyn Read> = get_reader(input)?;
//     let mut buffer = Vec::new();
//     reader.read_to_end(&mut buffer)?;

//     // 两种编码类型
//     let encoded = match format {
//         Base64Format::Standard => STANDARD.encode(buffer),
//         Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buffer),
//     };
//     println!("{}", encoded);
//     Ok(encoded)
// }
