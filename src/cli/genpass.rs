/* genpass基础方法, gen_pass是方法调用 */

use crate::{process::gen_pass::process_genpass, CmdExector}; // 可以直接访问 ../lib.rs的内容
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 20)]
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

// 为这个结构体实现接口Trait
impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret: String = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?; // 为什么这样调用
        println!("{}", ret);

        // output password strength in stderr
        let estimate = zxcvbn(&ret, &[])?;
        eprintln!("strength: {}", estimate.score());

        Ok(())
    }
}
