#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! 42 | 借用检查: 深入理解申明周期参数 T vs &T
//! https://time.geekbang.org/course/detail/100060601-322365
//!
//!
//!
//!

// cargo watch -x "test book_alex_programming::ch02::_42_T_refT::_a::main -- --nocapture"
mod _a {
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("`print`: t is {:?}", t);
    }

    fn print_ref<'a, T>(t: &'a T)
    where
        T: Debug + 'a,
    {
        println!("`print_ref`: t is {:?}", t);
    }

    #[test]
    fn main() {
        let x: i32 = 7;
        let ref_x = Ref(&x);
        print_ref(&ref_x);
        print(ref_x);
    }

    // 示例：Rust Quiz 5 ：[https://zhuanlan.zhihu.com/p/51616607](https://zhuanlan.zhihu.com/p/51616607)
    // 示例：来自于社区 Potato TooLarge 的案例
    // [https://zhuanlan.zhihu.com/p/194156624](https://zhuanlan.zhihu.com/p/194156624)


}