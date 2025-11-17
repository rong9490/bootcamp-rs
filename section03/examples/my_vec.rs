// // cargo run --example my_vec

// use anyhow::{Result};
// use section03::{my_vec, my_vec2};

// fn main() -> Result<()> {
//     // 三种括号都是等价的
//     let v: Vec<i32> = my_vec![1, 2, 3];
//     let v2 = my_vec!(4, 5, 6);
//     let v3 = my_vec! {7, 8, 9};
//     println!("{:?} {:?} {:?}", v, v2, v3);

//     println!("{:?}", my_vec2![]);

//     my_vec2![
//         "1".parse::<i32>()?,
//         "2".parse::<i32>()?,
//         "3".parse::<i32>()?,
//         "4".parse::<i32>()?,
//         "5".parse::<i32>()?, // 尾逗号
//     ];

//     // 源码写法: 先Box开辟内存, 再转为rev
//     let v3: Vec<i32> = <[_]>::into_vec(Box::new([1, 2, 3, 4]));

//     // print宏, tt 为 "single token tree"
//     print!("");
//     // macro_rules! print {
//     //     ($($arg:tt)*) => {{
//     //         $crate::io::_print($crate::format_args!($($arg)*));
//     //     }};
//     // }

//     Ok(())
// }

fn main() {}