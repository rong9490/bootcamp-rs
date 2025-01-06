/* 格式化, 涉及到: From Trait 和 Into Trait */

use std::{fmt, str::FromStr};

// pub trait FromStr: Sized {}

// 输出文件格式枚举
#[derive(Debug, Clone, Copy,)]
pub enum OutputFormat {
    Json,
    Yaml,
}

/* === FromStr Trait === */


/* === Into Trait === */