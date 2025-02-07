// 代码格式检查
#![warn(clippy::all, clippy::pedantic)]

// TODO legacy

use crate::claps::add;
use crate::async_book:add as async_book_add;

fn main() -> anyhow::Result<()> {
    println!("Hello, world! main.rs");
    // let result = add(2, 2);
    let result = async_book_add(2, 2);
    println!("Result: {}", result);
    Ok(())
}
