use std::path::Path;

// 输入文件校验
pub fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    let file_exists = Path::new(file_name).exists();
    if file_exists {
        if !file_name.ends_with(".csv") {
            return Err("输入文件必须是CSV文件");
        }
        return Ok(file_name.into());
    } else {
        return Err("输入文件不存在");
    }
}

// 校验目录存在
pub fn verify_dirpath(path: &str) -> Result<String, &'static str> {
    let p: &Path = Path::new(path);
    if p.exists() && p.is_dir() {
        return Ok(path.into());
    } else {
        return Err("目录不存在");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(
            verify_input_file("assets/juventus.csv"),
            Ok("assets/juventus.csv".into())
        );
        assert_eq!(verify_input_file("assets/a.csv"), Err("输入文件不存在"));
    }

    #[test]
    fn test_verify_path() {
        assert_eq!(verify_dirpath("assets"), Ok("assets".into()));
        assert_eq!(verify_dirpath("assets/juventus.csv"), Err("目录不存在"));
    }
}
