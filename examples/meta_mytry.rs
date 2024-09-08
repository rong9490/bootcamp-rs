use anyhow::anyhow;
use anyhow::Result;

// my_try! 宏 (simulate)
// 作用等同于 ? 操作符, 但是更简洁
macro_rules! my_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
}

// my_ready! 宏
// 用于模拟 Poll::Ready 和 Poll::Pending
#[macro_export]
macro_rules! my_ready {
    ($e:expr) => {
        match $e {
            std::task::Poll::Ready(v) => v,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}

// // ready 宏
// // 用于模拟 Poll::Ready
// #[macro_export]
// macro_rules! ready {
//     ($e:expr) => {
//         std::task::Poll::Ready($e)
//     };
// }

fn main() -> Result<()> {
    // std::ops::Try 操作符 ?
    // nightly-only experimental feature
    // Option 和 Result 都实现了 std::ops::Try, 但是不能混用!!
    let ret = f3(f2(f1("hello")?)?)?; // operator ? 操作符
    println!("Final result: {}", ret);

    let ret2 = my_try!(f3(my_try!(f2(my_try!(f1("hello")))))); // my_try! 宏
    println!("Final result: {}", ret2);
    Ok(())
}

// 每一个都可能报错
fn f1(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1: {}", s.as_ref()))
}

fn f2(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f2: {}", s.as_ref()))
}

fn f3(s: impl AsRef<str>) -> Result<String> {
    Err(anyhow!("f3: {}", s.as_ref()))
}
