// https://github.com/tyr-rust-bootcamp

use tiger::first_section::csv_cli_convert::major::major as major_csv;

fn main() -> anyhow::Result<()> {
    major_csv()?;
    Ok(())
}