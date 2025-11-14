// cmap.rs
// metrics data structure
// 基本功能: inc/dec/snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>, // Arc<Mutex<HashMap<String, i64>>> => Arc<DashMap<String, i64>>
}

// default::Default 默认值
impl Default for CmapMetrics {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

// 实例化 + 值增加
impl CmapMetrics {
    pub fn new() -> Self {
        CmapMetrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}


// 格式化输出
impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
