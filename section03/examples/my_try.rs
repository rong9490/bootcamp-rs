// // 3.2 20min
// use anyhow::{anyhow, Result};

// fn main() -> Result<()> {
//     // let ret = f3(f2(f1("hello")?)?)?;
//     let ret = my_try!(f3(my_try!(f2(my_try!(f1("hello"))))));
//     println!("Final result: {}", ret);
//     Ok(())
// }

// fn f1(s: impl AsRef<str>) -> Result<String> {
//     Ok(format!("f1: {}", s.as_ref()))
// }

// fn f2(s: impl AsRef<str>) -> Result<String> {
//     Ok(format!("f2: {}", s.as_ref()))
// }

// fn f3(s: impl AsRef<str>) -> Result<String> {
//     Err(anyhow!("f3: {}", s.as_ref()))
// }

// // 运算符? 是个 'nightly-only'的, 是无法被重载改写的, 只能用于 Result/Option, 两个情况, 但是不能混用!
// // 无法自定义数据结构中重载
// // std::stak::Polling / Pending; 
// // my_ready! => Poll::Ready / Pending;
// // 无法直接?处理异常, 只能用内部自带的错误处理方式
// // ? operator. How to simulate it?
// #[macro_export]
// macro_rules! my_try {
//     ($expr:expr) => {
//         match $expr {
//             Ok(val) => val,
//             Err(err) => return Err(err.into()),
//         }
//     };
// }

fn main() {}