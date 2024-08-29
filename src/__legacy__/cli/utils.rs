use std::path::{Path, PathBuf};

// 检查文件存在合法性
pub fn verify_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File does not exist: {}", filename))
    }
}

// 这里path是指文件目录(Folder)
pub(crate) fn verify_path(path: &str) -> Result<PathBuf, String> {
    // if input is "-" or file exists
    let p: &Path = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err(format!(
            "Path does not exist or is not a directory: {}",
            path
        ))
    }
}

// 养成习惯, 要配套测试用例

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into())); // "-"时直接成功返回
        assert_eq!(verify_file("*"), Err("File does not exist: *".into())); // 文件不存在
        assert_eq!(
            verify_file("not-exist"),
            Err("File does not exist: not-exist".into())
        ); // 文件不存在
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into())); // 文件存在
    }
}
