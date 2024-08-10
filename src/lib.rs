// pub mod cli;
// pub mod process;
// pub mod utils;
// use enum_dispatch::enum_dispatch;
// use cli::genpass::GenPassOpts;
// use cli::entry::SubCommand;
// use cli::csv::CsvOpts;
// use cli::base64::{Base64SubCommand, Base64EncodeOpts, Base64DecodeOpts};

// // pub use cli

// #[allow(async_fn_in_trait)]
// #[enum_dispatch] // 这个很关键
// pub trait CmdExector {
//     async fn execute(self) -> anyhow::Result<()>;
// }

pub mod csv_cli;