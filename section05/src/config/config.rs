use anyhow::{Error, Result, bail};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};

// 默认位置, 可以被环境变量覆盖
pub const DEF_APP_YAML_PATRH: &'static str = "chat.yaml";

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    // pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig, // 形成嵌套结构
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // 默认是父级目录: /Users/hejj/WorkSpace/Zed25/rust-hejj/axum_midend
        println!("{}", std::env::current_dir().unwrap().display());
        let ret: Result<AppConfig, serde_yaml::Error> = match (
            File::open("chat.yaml"),
            File::open("chat_room/chat.yaml"),
            File::open("/etc/config/chat.yaml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _, _) => serde_yaml::from_reader::<_, AppConfig>(reader),
            (_, Ok(reader), _, _) => serde_yaml::from_reader::<_, AppConfig>(reader),
            (_, _, Ok(reader), _) => serde_yaml::from_reader::<_, AppConfig>(reader),
            (_, _, _, Ok(path)) => serde_yaml::from_reader::<_, AppConfig>(File::open(path)?),
            _ => bail!("Config file not found!"),
        };
        Ok(ret?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
