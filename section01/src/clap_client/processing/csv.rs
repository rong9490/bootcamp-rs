use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::clap_client::clienting::csv::OutputFormat;

// 核心数据结构, 及其序列化+反序列化(自动Trait)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PlayerData {
    #[serde(rename = "Name")] // 域字段重命名
    pub name: String,

    #[serde(rename = "Position")]
    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    #[serde(rename = "Nationality")]
    nationality: String,

    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_player_serialize() {
        let player = PlayerData {
            name: String::from("张三"),
            position: String::from("前锋"),
            dob: String::from("1995-01-01"),
            nationality: String::from("中国"),
            kit: 10,
        };

        // 从Struct序列化为Json
        let json: String = serde_json::to_string(&player).unwrap();
        assert!(json.contains("张三"));
        assert!(json.contains("前锋"));
    }

    #[test]
    fn test_player_deserialize() {
        // 从Json字符串反序列化为Struct
        let json: &str = r#"
            {
                "Name": "李四",
                "Position": "中场",
                "DOB": "1996-02-02", 
                "Nationality": "中国",
                "Kit Number": 8
            }
        "#;

        let player: PlayerData = serde_json::from_str(json).unwrap();
        assert_eq!(player.name, "李四");
        assert_eq!(player.position, "中场");
        assert_eq!(player.kit, 8);
    }
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    println!("process_csv!");
    // todo!();
    Ok(())
}

// use anyhow::Result;
// use csv::Reader;
// use serde::{Deserialize, Serialize};
// use std::fs;

// use crate::clap_client::clienting::csv::OutputFormat;

// pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
//     let mut reader = Reader::from_path(input)?;
//     let mut ret = Vec::with_capacity(128);
//     let headers = reader.headers()?.clone();
//     for result in reader.records() {
//         let record = result?;
//         // headers.iter() -> 使用 headers 的迭代器
//         // record.iter() -> 使用 record 的迭代器
//         // zip() -> 将两个迭代器合并为一个元组的迭代器 [(header, record), ..]
//         // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
//         let json_value = headers
//             .iter()
//             .zip(record.iter())
//             .collect::<serde_json::Value>();

//         ret.push(json_value);
//     }

//     let content = match format {
//         OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
//         OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
//     };
//     fs::write(output, content)?;
//     Ok(())
// }

// println!("{} / {} / {}", format, input, output);
// let mut reader: Reader<File> = Reader::from_path(input)?;

// // 读取头部: 双重mut borrow, 需要clone消除错误
// // let _headers: StringRecord = reader.headers()?.clone();

// // 生成json对象, 固定大小
// let mut vec_player: Vec<Value> = Vec::with_capacity(128);

// // 迭代器+闭包写法: let vec_player: Vec<Player> = reader.deserialize().map(|record| record.unwrap()).collect::<Vec<Player>>();

// // 暂时不 reader.deserialize(), 限定了数据结构 不够通用 / StringRecord不包含header的key, 需要用迭代器生成对象
// for result in reader.records() {
//     let record = result?;
//     let player: Value = record.into_iter().collect();
//     vec_player.push(player);
// }

// // 按格式分发处理
// let content: String = match format {
//     OutputFormat::Json => serde_json::to_string_pretty(&vec_player)?,
//     OutputFormat::Yaml => serde_yaml::to_string(&vec_player)?,
// };

// // 最后, 写入文件
// fs::write(output.clone(), content)?;
// println!("文件已写入完成! {}", output);

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_csv_convert_json() {
//         let input: String = String::from("assets/juventus.csv");
//         let output: String = String::from("assets/juventus.json");
//         let format: OutputFormat = OutputFormat::Json;

//         // let result: Result<(), Error> = csv_convert(format, input, output);
//         // assert!(result.is_ok());
//     }

//     #[test]
//     fn test_csv_convert_yaml() {
//         let input: String = String::from("assets/juventus.csv");
//         let output: String = String::from("assets/juventus.yaml");
//         let format: OutputFormat = OutputFormat::Yaml;

//         // let result: Result<(), Error> = csv_convert(format, input, output);
//         // assert!(result.is_ok());
//     }
// }
