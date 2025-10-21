#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! 35 | 所有权: Copy语义与Copy trait
//! https://time.geekbang.org/course/detail/100060601-316828
//!
//! 复习: 位置表达式 & 值表达式

mod _1 {

}


// Rust枚举, 构造器, 带数据
// 无法直接给Person实现Copy, 为什么?
// 包含 name: &str

// std::marker::Copy
// std::clone::Clone clone / clone_from 叫做 trait集成/限定
// 为什么是空的, 编译器行为, 标记📌作用
// 打印, 隐式调用 let b = a.clone();
//
// 如果自行实现 impl Clone for A {} --> 编译器默认并不会调用, 需要显式调用
// 编译器实现的clone一定会覆盖你的clone, 除非你显式调用(无关)
// 无法修改编译器行为 --> 按位复制
// TODO 补充代码
