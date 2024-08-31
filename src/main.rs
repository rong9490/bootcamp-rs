// https://github.com/tyr-rust-bootcamp

use tiger::first_section::clap_cli::major::major as major_clap;

fn main() -> anyhow::Result<()> {
    major_clap()?;
    Ok(())
}
