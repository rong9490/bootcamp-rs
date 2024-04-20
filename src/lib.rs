// // mod opts;
// mod cli;
// mod process;

// pub use process::{process_csv, process_genpass};

mod cli;
mod process;
mod utils;

use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)] // TODO ??
#[enum_dispatch] // TODO 动态分发: 自动实现, 速度更快: 宏 + 特型trait(语法糖)
pub trait CmdExector {
  async fn execute(self) -> anyhow::Result<()>;
}