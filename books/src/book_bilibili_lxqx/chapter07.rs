// cargo run --example lesson_lxqx07
/*
 * 7.引用和生命周期
 * 可变引用 & 不可变引用 不同在
 */

use std::collections::HashMap;

#[derive(Debug)]
struct QueryParser {
    query: String,
    params: HashMap<String, String>,
}

// 第一种拆分方式: 不使用引用, 效率更低, 但简单好理解
impl QueryParser {
    fn new_from_string(query: String) -> Self {
        // eg. name=joker&age=20&gender=true 暂时只用字符串
        let pairs = query.split("&").map(|item| {
            let mut parts = item.split("=");
            let key: String = String::from(parts.next().unwrap()); // None.unwrap();
            let value: String = String::from(parts.next().unwrap());
            (key, value)
        });

        // 元组迭代器转HashMap
        let params: HashMap<String, String> = pairs.collect::<HashMap<_, _>>();
        Self { query, params }
    }
}

// 升级为引用版 --> 考虑生命周期
struct QueryParserRef<'a> {
    query: &'a str, // missing lifetime specifier
    params: HashMap<&'a str, &'a str>,
}

impl<'a> QueryParserRef<'a> {
    fn new_from_string(query: &'a str) -> Self {
        let pairs = query.split("&").map(|item| {
            let mut parts = item.split("=");
            let key: &'a str = parts.next().unwrap(); // None.unwrap();
            let value: &'a str = parts.next().unwrap();
            (key, value)
        });

        let params: HashMap<&'a str, &'a str> = pairs.collect::<HashMap<_, _>>();
        Self { query, params }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() -> () {
        let list: Vec<i32> = Vec::new();
        let size_of: usize = std::mem::size_of::<Vec<i32>>();
        println!("size_of: {}", size_of);
        // list.push(); // 查看签名: push(&mut self, value: T)
    }

    #[test]
    fn test_query_parser01() -> () {
        let query_str: &'static str = "name=joker&age=20&gender=true";
        let query_parser: QueryParser = QueryParser::new_from_string(query_str.to_string());
        println!("{:?}", query_parser);

        // 两种比较方式 ✅
        assert_eq!(
            *query_parser.params.get("name").unwrap(),
            "joker".to_string()
        );
        assert_eq!(query_parser.params.get("age"), Some(&"20".to_string()));
    }
}
