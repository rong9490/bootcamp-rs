use enum_dispatch::enum_dispatch;

// TODO 每个子命令都需要实现各自的执行器, 供父级枚举调用(SubCommand)
// #[allow(async_fn_in_trait)]
// #[enum_dispatch]
// pub trait CmdExector {
//     async fn execute(self) -> anyhow::Result<()>;
// }

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
