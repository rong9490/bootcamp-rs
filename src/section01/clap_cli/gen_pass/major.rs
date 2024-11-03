use clap::Parser;
use rand::{thread_rng, Rng};

#[derive(Debug, Parser)]
pub struct GenPassSub {
    #[arg(long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

// 副命令: 随机密码
pub fn major_clap_gen_pass(
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
    length: u8,
) -> anyhow::Result<()> {
    let mut rng = thread_rng();
    let mut password = String::new();
    let mut chars: Vec<char> = Vec::new();

    if uppercase {
        chars.extend_from_slice(&('A'..='Z').collect::<Vec<char>>());
    }
    if lowercase {
        chars.extend_from_slice(&('a'..='z').collect::<Vec<char>>());
    }
    if number {
        chars.extend_from_slice(&('0'..='9').collect::<Vec<char>>());
    }
    if symbol {
        chars.extend_from_slice(&('!'..='~').collect::<Vec<char>>());
    }
    if chars.is_empty() {
        anyhow::bail!("至少选择一种字符类型");
    }

    for _ in 0..length {
        // 随机堆入, 直到长度足够
        password.push(chars[rng.gen_range(0..chars.len())]);
    }

    // eprintln!
    println!("pass: {}", password);

    // TODO 验证密码强度 zxcvbn

    Ok(())
}
