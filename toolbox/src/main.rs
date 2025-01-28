use anyhow::{Result, Ok};

use claps::major::major as claps_major;

fn main() -> Result<()> {
    claps_major()?;
    Ok(())
}
