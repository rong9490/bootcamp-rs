mod cli;
mod process;
mod utils;

pub use cli::*; // 现在还剩这里面没有完成搭建了
pub use utils::*;
pub use process::*;
use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)] // TODO ??
#[enum_dispatch] // TODO 动态分发: 自动实现, 速度更快: 宏 + 特型trait(语法糖)
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
