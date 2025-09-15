use enum_dispatch::enum_dispatch;
use crate::clap_client::clienting::{csv::CsvOpts, genpass::GenPassOpts};
use crate::clap_client::clienting::SubCommand;

// 每个子命令都需要实现各自的执行器, 供父级枚举调用(解耦)
#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}

// use super::clienting::Opts;
// use crate::clap_client::_01_csv_convert::output_format::OutputFormat;
// use std::path::Path;

// /// csv处理完的输出文件名
// pub fn get_csv_output_filename(output: Option<String>, format: OutputFormat) -> String {
//     let output: String = if let Some(output) = output {
//         output.clone()
//     } else {
//         format!("default_output.{}", format) // 缺省输出文件名
//     };
//     output
// }

// // 校验文件存在与否(带后缀)
// // Result<String, String / &'static str> 暂时兼容对外
// pub fn verify_file_exists_with_ext(file_name: &str, ext: &str) -> Result<String, &'static str> {
//     let p: &Path = Path::new(file_name);
//     if p.exists() {
//         if !file_name.ends_with(ext) {
//             return Err(Box::leak(
//                 format!("输入文件扩展名不正确，应为{}", ext).into_boxed_str(),
//             ));
//         }
//         Ok(file_name.into())
//     } else {
//         Err("输入文件不存在")
//     }
// }

// // 校验文件存在与否
// pub fn verify_file_exists(file_name: &str) -> Result<String, &'static str> {
//     let p: &Path = Path::new(file_name);
//     if p.exists() {
//         if !file_name.ends_with(".csv") {
//             return Err("输入文件必须是CSV文件");
//         }
//         Ok(file_name.into())
//     } else {
//         Err("输入文件不存在")
//     }
// }

// // 校验目录存在与否
// pub fn verify_dirpath_exists(path: &str) -> Result<String, &'static str> {
//     let p: &Path = Path::new(path);
//     if !p.exists() {
//         Err("目录不存在")
//     } else if !p.is_dir() {
//         Err("输入参数不是目录")
//     } else {
//         Ok(path.into())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_verify_file_exists() {
//         const EXISTS_FILE: &str = "assets/juventus.csv";
//         const NOT_EXISTS_FILE: &str = "assets/xxyy.csv";
//         // 打印当前运行的基础路径，类似于 Node.js 的 process.cwd()
//         let cwd = std::env::current_dir().unwrap(); // std::fs::canonicalize(EXISTS_FILE);
//         println!("cwd: {:?}", cwd);
//         assert_eq!(verify_file_exists(EXISTS_FILE), Ok(EXISTS_FILE.into()));
//         assert_eq!(verify_file_exists(NOT_EXISTS_FILE), Err("输入文件不存在"));
//     }

//     #[test]
//     fn test_verify_dirpath_exists() {
//         const EXISTS_DIR: &str = "assets";
//         const NOT_DIR: &str = "assets/juventus.csv";
//         const NOT_EXISTS_DIR: &str = "assets_xxyy";

//         assert_eq!(verify_dirpath_exists(EXISTS_DIR), Ok(EXISTS_DIR.into()));
//         assert_eq!(verify_dirpath_exists(NOT_DIR), Err("输入参数不是目录"));
//         assert_eq!(verify_dirpath_exists(NOT_EXISTS_DIR).is_err(), true);
//     }
// }
