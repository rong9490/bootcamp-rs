/* "密码生成" 副命令集群 */
// 命令: cargo run genpass --length 50
use crate::{process::genpass::process_genpass, CmdExector};
use clap::Parser;
use zxcvbn::{zxcvbn, Entropy};

#[derive(Debug, Parser)]
// (副命令)案例一: 密码生成器
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 30)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

// 为GenPassOpts这个静态的结构体, 实现CmdExector这个Trait接口方法
// 这里不进行具体的执行代码, 交给 process_genpass
impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password: String = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("密码生成器 {}", password);

        let score: u8 = self.score(password)?;
        println!("密码得分 {}", score);
        Ok(())
    }
}

impl GenPassOpts {
    fn score(self, password: String) -> anyhow::Result<u8> {
        let estimate: Entropy = zxcvbn(&password, &[])?;
        let score: u8 = estimate.score();
        Ok(score)
    }
}
