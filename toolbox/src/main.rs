use anyhow::{Result, Ok};

// use claps::major::major as claps_major;
use async_book::add as async_book_add;

fn main() -> Result<()> {
    // claps_major()?;
    let result = async_book_add(1, 3);
    println!("{}", result);
    Ok(())
}
