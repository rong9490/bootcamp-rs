use anyhow;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

// 定义单位数据结构
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // 驼峰命名法? 只需要对不符合的单独rename
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String, // DateTime<Utc>
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input).unwrap();
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result.unwrap();
        // println!("{:?}", record);
        ret.push(record);
    }
    // 处理成json
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
