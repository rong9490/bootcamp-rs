#![allow(unused_variables)]

//! 07 | 语法面面观: 面向表达式(二)
//! https://time.geekbang.org/course/detail/100060601-289994

//! CTFE 编译期函数求值 （compile time function evaluation）
//! 什么是编译器计算 ?
//! 常量泛型(const generic)
//!
//!
//!
//! if / match / loop / println 引应用常量函数
//! crates.io 搜索 const-sha1库实现源码
//! std::vec::Vec的new方法, 为什么是常量函数, 有什么好处?

pub fn const_generic_show() {
    println!(" 1.4 : Const Generic Show")
}

/// 1.4: FizzBuzz in match
mod fizzbuzz_in_match {
    fn fb_in_match() -> () {
        for i in 1..102 {
            match (i%3, i%5) {
                (0, 0) => println!("FizzBuzz"),
                (0, _) => println!("Fizz"),
                (_, 0) => println!("Buzz"),
                (_, _) => println!("{}", i)
            }
        }
    }

    fn fb_in_if() -> () {
        for i in 1..102 {
            if i % 15 == 0 { println!("FizzBuzz") }
            else if i % 3 == 0 { println!("Fizz") }
            else if i % 5 == 0 { println!("Buzz") }
            else { println!("{}", i) }
        }
    }
}
