/* 通用方法 */

use std::path::{Path, PathBuf};

pub(crate) fn verify_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File does not exist: {}", filename)) // HACK 感觉可以封装所有的错误类型及其行为
    }
}

// 这里path是指文件目录(Folder)
// TODO 这里返回错误信息  &'static str 还是 String 有多大区别
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