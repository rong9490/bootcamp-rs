use crate::cli::OutputFormat;
use anyhow;
use csv::{Reader, StringRecord};
use serde::{Deserialize, Serialize};
use std::fs;

/* 定义单位数据结构 */
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // 驼峰命名法, 只需要对不符合的单独rename
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

/* 正式: 执行数据转换 */
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    // 创建文件的读取器
    let mut reader = Reader::from_path(input)?;

    // 装数据的容器, 暂定最多128条
    let mut ret = Vec::with_capacity(128);

    // 取出文件里的headers, 既读又写所以clone一份
    let headers = reader.headers()?.clone();

    // 开始遍历取出记录处理
    for result in reader.records() {
        let record: StringRecord = result?;

        // 遍历"表头"来取数, 重新组合成迭代器
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    // 对应不同的具体格式化
    let content: String = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    // 最终写入文件
    fs::write(output, content)?;
    Ok(())
}
