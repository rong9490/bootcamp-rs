// 代码格式检查
#![warn(clippy::all, clippy::pedantic)]

/* 课程地址: https://github.com/tyr-rust-bootcamp */

use toolbox::section01_clapcli::major::major as major_clapcli;

fn main() -> anyhow::Result<()> {
    major_clapcli()?;
    Ok(())
}
