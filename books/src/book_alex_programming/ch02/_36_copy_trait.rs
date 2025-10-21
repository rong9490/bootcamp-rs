#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! 36 | 所有权: 深入理解Copy行为
//! https://time.geekbang.org/course/detail/100060601-316828
//!
//!
//!

// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024

// 按位复制:
mod _1 {
    #[derive(Copy, Clone)]
    struct A(i8, i32);
    fn main() {
        let a = A(1, 2);
        let b = a; // 按位复制，复制后，b和a完全相同，包括内存对齐填充的padding部分
        let c = A(a.0, a.1); // 逐成员复制，非按位复制，c和a的padding部分不一定相同

        // 逐个按成员复制 --> 非按位复制
    }

    fn unsound_a() -> A {
        #[derive(Debug, Copy, Clone)]
        struct B {
            a: u16,
            b: u8,
            c: u8,
        }
        // 依次修改 c 的值为 0，1，2 打印输出结果
        let b = B { a: 1, b: 1, c: 1 };
        unsafe { *(&b as *const B as *const A) } // HINT unsafe块, 布局相同

        // soma_a = None
    }
}

mod _2 {
    #![allow(unused_variables)]

    use std::{mem, ptr};

    #[test]
    fn main() {
        let mut d = String::from("cccc");
        let d_len = d.len();
        let mut c = String::with_capacity(d_len);

        // 花括号开辟子作用域: 源指针 -> 到 目标指针

        unsafe {
            ptr::copy(&d, &mut c, 1);
        };
        println!("{:?}", c.as_ptr());
        // unsafe {
        //     ptr::drop_in_place(c.as_mut_ptr());
        // }
        // 注掉 drop，会产生double free，
        // 但是不注掉 drop，会产生无效指针
        mem::drop(c);

        println!("{:?}", d.as_ptr());
        d.push_str("c");
        println!("{}", d);

        // HINT:
        // free(): double free detected in tcache 2  -->  检测出未定义行为!
        // timeout: the monitored command dumped core
    }
}
