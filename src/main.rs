#![warn(clippy::all, clippy::pedantic)]

// https://github.com/tyr-rust-bootcamp

// use tiger::section01::clap_cli::major::major as major_clap;
use tiger::programming::major::major as major_pg;

fn main() -> anyhow::Result<()> {
    // major_clap()?;
    major_pg();
    Ok(())
}
