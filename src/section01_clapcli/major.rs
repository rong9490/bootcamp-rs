// 当前这个模块mod的唯一出口, 组织子功能模块

use anyhow::{Result, Ok};
use super::clap_cli::major::major as major_clap_cli;

pub fn major() -> Result<()> {
    println!("第一课: 父模块 ClapCli");

    // 决定执行哪个模块! 后续考虑做成环境变量或者自动化
    let flag: u8 = 1;
    if flag == 1 {
        major_clap_cli()?;
    }
    Ok(())
}
