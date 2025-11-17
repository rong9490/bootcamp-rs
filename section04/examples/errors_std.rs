// 自动转换
//  impl From<Err1> for Err3 {
//     fn from(v: Err1) -> Err3 {}
//  }
// match ret {
//     Ok(v) => v,
//     Err(e) => return Err(e.into()),
// }

/// 先查看和理解, 核心库的: Trait std::error::Error
/// pub trait Error: Debug + Display {
///     fn source() { ... } --> 从哪里来的
///     fn description() { ... } --> x 已弃用
///     fn cause() { ... }
///     fn provide() { ... } --> Provider提供方
/// }
/// Trait Error: 必须实现 Debug 和 Display
///
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SuperError {
    source: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
struct SuperErrorSideKick;

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl Error for SuperErrorSideKick {}

fn main() -> () {}
