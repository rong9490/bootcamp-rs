#![warn(clippy::all, clippy::pedantic)]

// https://github.com/tyr-rust-bootcamp

use tiger::section01::clap_cli::major::major as major_clap;

fn main() -> anyhow::Result<()> {
    major_clap()?;
    Ok(())
}
