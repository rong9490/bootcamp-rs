use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)] // 实现 序列化, 反序列化
pub struct Player {
    #[serde(rename = "Name")] // 对域进行重命名
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}