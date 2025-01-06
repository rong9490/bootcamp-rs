use serde::{Deserialize, Serialize};

/* 球员 数据结构(序列化, 反序列化) */

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // 自动转换驼峰命名
pub struct PlayerDate {
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
        let player = PlayerDate {
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

        let player: PlayerDate = serde_json::from_str(json).unwrap();
        assert_eq!(player.name, "李四");
        assert_eq!(player.position, "中场");
        assert_eq!(player.kit, 8);
    }
}
