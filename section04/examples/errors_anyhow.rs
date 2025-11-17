use anyhow::Context;
use std::{fs, mem::size_of, num::ParseIntError};
use thiserror::Error;

// 错误枚举
#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize json error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Error: {0:?}")]
    BigError(Box<BigError>),
    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: u64,
}

// cargo watch -x "run --package section04 --example errors_anyhow"
fn main() -> Result<(), anyhow::Error> {
    println!("size of anyhow::Error is {}", size_of::<anyhow::Error>()); // 指针8bytes
    println!("size of std::io::Error is {}", size_of::<std::io::Error>()); // 指针8bytes
    println!(
        "size of std::num::ParseIntError is {}",
        size_of::<ParseIntError>()
    );
    println!(
        "size of serde_json::Error is {}",
        size_of::<serde_json::Error>()
    );
    println!("size of string is {}", size_of::<String>());
    println!("size of MyError is {}", size_of::<MyError>());

    // 故意报错查看堆栈"Stack backtrace"
    let filename: &str = "non-existent-file.txt";

    // anyhow::context::Result::with_context 自动将 io::Result<File> 转为 anyhow::Error
    // Wrap the error value with additional context that is evaluated lazily only once an error does occur.
    // 将错误值与其上下文一并封装, 这些上下文仅在错误发生时才会被延迟惰性调用!
    let _fd: fs::File =
        fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
