#![warn(clippy::all, clippy::pedantic)]

// https://github.com/tyr-rust-bootcamp

// use tiger::section01::clap_cli::major::major as major_clap;
use tiger::section02::concurrent::major::major as major_concurrent;
// use tiger::programming::major::major as major_pg;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init(); // NOTE 初始化日志
    // major_clap()?;
    // major_pg();
    major_concurrent();
    Ok(())
}
