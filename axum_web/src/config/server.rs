use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    port: Option<u16>
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        // 默认值
        self.port.unwrap_or(4000)
    }
}