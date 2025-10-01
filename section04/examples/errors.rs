/** anyhow / thiserror 的使用 */
// cargo run --example errors
use std::{fs, mem::size_of, mem::size_of_val, num::ParseIntError};
use anyhow::{Context, Result};
use thiserror::Error;

// 官方文档介绍:
// fn get_cluster_info() -> Result<ClusterMap> {
//   let config: String = std::fs::read_to_string("cluster.json")?;
//   let map: ClusterMap = serde_json::from_str(&config);
//   Ok(map)
// }d

// 上下文context用法
// fn main() -> Result<()> {
//   it.detach().context("Failed to detach the important thing")?;
//   let content = std::fs::read(path).with_context(|| format!("Failed to read instrs from {}", path));
// }

// match root_cause.downcast_ref::<DataStoreError> {
//   Some(DataStoreError::Censored(_)) => Ok(Poll::Ready(REDACTED_CONTENT))
//   None => Err(error),
// }

#[derive(Error, Debug)]
pub enum MyError {
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),
  #[error("ParseInt error: {0}")]
  Parse(#[from] std::num::ParseIntError),
  #[error("Serialize json error: {0}")]
  Serialize(#[from] serde_json::Error),
  #[error("Error: {0:?}")]
  BigError(Box<BigError>),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
  a: String,
  b: Vec<String>,
  c: [u8; 64],
  d: u64,
}

fn main() {
  println!("Okk!");
}
