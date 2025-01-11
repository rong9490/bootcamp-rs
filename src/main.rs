// 代码格式检查
#![warn(clippy::all, clippy::pedantic)]

/* 课程地址: https://github.com/tyr-rust-bootcamp */

use toolbox::section01_clapcli::major::major as major_clapcli;
use toolbox::utils::env::EnvEntity;

fn main() -> anyhow::Result<()> {
    let env_entity: EnvEntity = EnvEntity::new()?;
    if env_entity.tb_feat_flag == 1 {
        major_clapcli()?;
    }
    Ok(())
}
