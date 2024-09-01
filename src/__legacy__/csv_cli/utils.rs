use std::path::Path;

// 验证文件是否存在
pub fn verify_file_exists(filename: &str) -> Result<String, &'static str> {
    let exists: bool = Path::new(filename).exists();
    if exists {
        Ok(filename.into())
    } else {
        Err("文件不存在!")
    }
}