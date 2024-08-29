use anyhow::Result;
use std::{fs::File, io::Read};

// "_" 特殊处理:
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "_" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

// 读取内容
pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

// // 执行器Trait
// #[allow(async_fn_in_trait)]
// #[enum_dispatch]
// pub trait CmdExector {
//     async fn execute(self) -> anyhow::Result<()>;
// }
