use clap::Parser;

use crate::clap_client::utils::CmdExector;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, default_value_t = 16)]
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

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret =crate::clap_client::processing::genpass::csv_process(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;

        // 检验随机密码的强度, 考虑缓存写下来
        let estimate = zxcvbn(&ret, &[])?;
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
