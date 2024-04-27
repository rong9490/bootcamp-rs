pub mod cli;
pub mod process;
pub mod utils;
use enum_dispatch::enum_dispatch;
use cli::genpass::GenPassOpts;
use cli::entry::SubCommand;
use cli::csv::CsvOpts;

// pub use cli

#[allow(async_fn_in_trait)]
#[enum_dispatch] // 这个很关键
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
