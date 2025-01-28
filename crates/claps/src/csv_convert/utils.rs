use anyhow::Result;
use std::path::Path;

// 校验文件存在与否
pub fn verify_file_exists(file_name: &str) -> Result<String, &'static str> {
    let p: &Path = Path::new(file_name);
    if p.exists() {
        if !file_name.ends_with(".csv") {
            return Err("输入文件必须是CSV文件");
        }
        Ok(file_name.into())
    } else {
        Err("输入文件不存在")
    }
}

// 校验目录存在与否
pub fn verify_dirpath_exists(path: &str) -> Result<String, &'static str> {
    let p: &Path = Path::new(path);
    if !p.exists() {
        Err("目录不存在")
    } else if !p.is_dir() {
        Err("输入参数不是目录")
    } else {
        Ok(path.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        const EXISTS_FILE: &str = "assets/juventus.csv";
        const NOT_EXISTS_FILE: &str = "assets/xxyy.csv";

        assert_eq!(verify_file_exists(EXISTS_FILE), Ok(EXISTS_FILE.into()));
        assert_eq!(verify_file_exists(NOT_EXISTS_FILE), Err("输入文件不存在"));
    }

    #[test]
    fn test_verify_dirpath_exists() {
        const EXISTS_DIR: &str = "assets";
        const NOT_DIR: &str = "assets/juventus.csv";
        const NOT_EXISTS_DIR: &str = "assets_xxyy";

        assert_eq!(verify_dirpath_exists(EXISTS_DIR), Ok(EXISTS_DIR.into()));
        assert_eq!(verify_dirpath_exists(NOT_DIR), Err("输入参数不是目录"));
        assert_eq!(verify_dirpath_exists(NOT_EXISTS_DIR).is_err(), true);
    }
}
