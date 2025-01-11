/* 集中处理环境变量 */

use anyhow;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct EnvEntity {
    pub tb_feat_flag: u8,
    pub tb_sftp_host: String,
}

impl EnvEntity {
    pub fn new() -> anyhow::Result<Self> {
        dotenv().ok(); // 记载.env文件

        // 获取所有环境变量并转换为Config结构体
        let entity =  EnvEntity {
            tb_feat_flag: env::var("TB_FEAT_FLAG")
                .unwrap_or_else(|_| "0".to_string())
                .parse()
                .unwrap_or(0), // 默认值0
            tb_sftp_host: env::var("TB_SFTP_HOST")?,
        };
        anyhow::Ok(entity)
    }
}
