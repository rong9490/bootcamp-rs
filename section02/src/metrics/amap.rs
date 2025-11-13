// amap.rs

use anyhow::Result;
use std::{
    collections::HashMap,
    fmt,
    hash::RandomState,
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
};

// AmapMetrics = Atomic Metrics 原子化的数据结构
#[derive(Debug)]
pub struct AmapMetrics {
    pub data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    // 内存占用
    pub fn mem_size_of() -> usize {
        std::mem::size_of::<AmapMetrics>() // Arc指针占内存8bytes
    }

    pub fn mem_size_val(&self) -> usize {
        let size_of_val: usize = std::mem::size_of_val(&self); // Arc指针占内存8bytes
        size_of_val
    }

    // 实例化
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map: HashMap<&'static str, AtomicI64> = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect::<HashMap<&'static str, AtomicI64>>();

        // 包裹一层原子化计数引用 "Atomic Ref Count"
        let arc_map: Arc<HashMap<&'static str, AtomicI64>> = Arc::new(map);
        Self { data: arc_map }
    }

    pub fn new_mock() -> Self {
        let metrics: AmapMetrics = AmapMetrics::new(&[
            "call.thread.worker.0",
            "call.thread.worker.1",
            "req.page.1",
            "req.page.2",
            "req.page.3",
            "req.page.4",
        ]);
        metrics
    }

    // 注意这里 impl AsRef<str>
    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key: &str = key.as_ref();
        let map: Arc<HashMap<&'static str, AtomicI64>> = self.data.clone(); // 获取Arc的只读引用
        let counter: Option<&AtomicI64> = map.get(key);

        // 将Option::None转为错误Result::Error
        let counter: Result<&AtomicI64, anyhow::Error> =
            counter.ok_or_else(|| anyhow::anyhow!("key {} not found!", key));
        let counter: &AtomicI64 = counter?; // 自动抛出
        // 计数器加1, 底层原语需要顺序(严格程度)
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo watch -x "test --package section02 --lib -- metrics::amap::tests::test01 --exact --nocapture"
    #[test]
    fn test01() -> () {
        let size_of: usize = AmapMetrics::mem_size_of();
        assert_eq!(size_of, 8);
        println!("size_of={}", size_of);
    }

    #[test]
    fn test02() -> () {
        let metrics: AmapMetrics = AmapMetrics::new_mock();
        let size_of_val: usize = metrics.mem_size_val();
        assert_eq!(size_of_val, 8);
        println!("size_of_val={}", size_of_val);
    }
}
