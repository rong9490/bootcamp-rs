use anyhow;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    // &StringRecord 类型
    let headers = reader.headers()?.clone(); // 读出header, 是否需要clone

    // 这里为什么要改成 reader.records()?
    // 这里既读又写, mut 会报错 --> 所以需要clone复制一份
    for result in reader.records() {
        let record = result?;
        // 熟悉: iter() / zip() / collect / 还有这里为什么是Map

        // TODO 这行代码比较重要: 为什么这样写链式操作 函数式代码
        // iter() 使用迭代器
        // zip 是将两个迭代器合并成元组 [(header, record), ...]
        // collect::<serde_json::Value>() 也实现了迭代器, Map里面
        // string 作为Key; Value
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        // let iter = headers.iter().zip(record.iter());

        // // TODO 这里需要不同的处理
        // let json_value = match format {
        //     OutputFormat::Json => iter.collect::<Value>(),
        //     OutputFormat::Yaml => todo!(),
        //     OutputFormat::Toml => todo!(),
        // };

        // 如果for循环还是要用zip: 麻烦

        ret.push(json_value);
    }

    // HACK 对应不同的具体格式化
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => toml::to_string(&ret)?, // 暂不支持toml
    };

    fs::write(output, content)?;
    Ok(())
}
