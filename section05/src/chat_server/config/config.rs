use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use anyhow::{Error, Result, bail};

pub const DEF_APP_YAML_PATRH: &'static str = "chat.yaml"; // 默认配置文件位置, 允许被环境变量覆盖

// '状态数据' 嵌套结构:  AppConfig -> ServerConfig
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    // pub host: String, '0.0.0.0'
    pub port: u16,
}

impl AppConfig {
    /** 实例化(load) */
    pub fn load() -> Result<Self> {
        // 默认父级目录(当前执行目录): pwd
        println!("{}", std::env::current_dir().unwrap().display());

        // 一次性读取4个文件, 可选优化: 先判断exists存在, 再读取文件
        let ret: Result<AppConfig, serde_yaml::Error> = match (
            File::open(DEF_APP_YAML_PATRH),
            File::open("chat_room/chat.yaml"), // 简化掉
            File::open("/etc/config/chat.yaml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _, _) => serde_yaml::from_reader::<_, AppConfig>(reader),
            (_, Ok(reader), _, _) => serde_yaml::from_reader::<_, AppConfig>(reader),
            (_, _, Ok(reader), _) => serde_yaml::from_reader::<_, AppConfig>(reader),
            (_, _, _, Ok(path)) => serde_yaml::from_reader::<_, AppConfig>(File::open(path)?),
            _ => bail!("AppConfig file not found!!"), // bail等于 return Err(anyhow!($args));
        };
        Ok(ret?)
    }
    /** 尝试实例化(try_load) */
    pub fn try_load() -> Result<Self> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // cargo watch -x "test --package section05 --lib -- config::config::tests::test --exact --nocapture"
    fn test() {
        let app_config = AppConfig::load();
        println!("is_ok: {}", app_config.is_ok());
    }
}
