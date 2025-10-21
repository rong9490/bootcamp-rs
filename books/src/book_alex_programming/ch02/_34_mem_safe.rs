#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! 32 | 所有权: 内存管理基础知识
//!
//!
//! C: 纯手工管理内存 (缺乏安全抽象模型)
//! C++: 手工+确定性析构 (缺乏安全抽象模型)
//! GC语言: 垃圾回收, py, js (性能差)
//!
//! Rust内存管理:
//! 1. 考虑性能: 借鉴Cpp的RAII资源管理方式 (半自动化 资源获取即初始化, 确定性析构)
//! 2. 考虑安全: 增加所有权语义(类型系统)
//! 5个方面, 一致性, 心智模型
//!
//! 仿射变换(Affine Type)理论依据
//! 资源最多只能被使用一次(被消耗掉了)
//! 对现实资源的抽象
//! TODO 32小节

/// Rust 语义：Move 语义 与 Copy 语义
/// 基本数据类型： https://doc.rust-lang.org/std/index.html#primitives
mod primitives {
    #[test]
    fn primitives_test() {
        {
            let answer = "42"; // 创建变量 与 绑定内存地址; 引用, 借用
            let no_answer = answer;

            let name = String::from("Joker");
            let no_name = name;

            // 通过生成 HIR / MIR 查看区别!
            // let mut _0: ();
            // scope 1 {
            //     debug answer => const "42";
            //     let _1: &str;
            //     scope 2 {
            //         debug no_answer => _1;
            //         let _2: std::string::String;
            //         scope 3 {
            //             debug name => _2;
            //             let _3: std::string::String;
            //             scope 4 {
            //                 debug no_name => _3;
            //             }
            //         }
            //     }
            // }
        }

        // impl Copy for i32
        let a = 42;
        let b = a;
        println!("{:?}", a); // work

        // impl Copy for &'static str
        let a = "42";
        let b = a;
        println!("{:?}", a); // work

        // impl !Copy for String
        let a = "42".to_string();
        // &String deref to &str
        let b: &str = &a;
        // impl Copy for &'a T
        let c = b;
        println!("{:?}", b); // work

        // impl !Copy for String
        let mut a = "42".to_string();
        // impl !Copy for &mut T
        let b: &mut str = &mut a;
        let c = b;
        // println!("{:?}", b); // don't work, b have been moved

        // auto impl Copy for Tuple, if all item implemented Copy trait in Tuple
        let t = (42, "42");
        let t2 = t;
        println!("{:?}", t); // work

        // auto impl !Copy for Tuple
        let t = (42, "42".to_string());
        let t2 = t;
        // println!("{:?}", t); // don't work, t have been moved
    }

    // 自定义数据类型
    #[test]
    fn struct_test() {
        #[derive(Debug, Copy, Clone)]
        struct A; // 0大小, 占位

        #[derive(Debug, Copy, Clone)]
        struct Point(u32);

        #[derive(Debug, Copy, Clone)]
        struct Member {
            name: &'static str,
            age: u32,
        }

        // #[derive(Debug, Copy, Clone)]
        struct Person {
            name: String, // TODO 无法实现CopyTrait
            age: u32,
        }
        {
            let a = A;
            let b = a;
            println!("{:?}", a); // work
        }
        {
            let a = Point(60);
            let b = a;
            println!("{:?}", a); // work
        }
        let a = Member {
            name: "Alex",
            age: "18".parse::<u32>().unwrap(),
        };
        let b = a;
        let a = Member {
            name: "Alex", // to_string()
            age: "18".parse::<u32>().unwrap(),
        };
        let b = a;
    }
}

// 理解按位复制: Copy
mod understand_copy_clone {
    struct A;

    // 没用，自己实现Copy和Clone无法改变编译器默认行为
    impl Clone for A {
        fn clone(&self) -> Self {
            println!("from Custom Copy: Clone");
            *self
        }
    }

    impl Copy for A {}

    #[test]
    fn main() {
        let a = A;
        let b = a;
    }

    mod understand_copy_clone_2 {
        #[derive(Copy, Clone)]
        struct A(i8, i32);
        fn main() {
            let a = A(1, 2);
            let b = a; // 按位复制，复制后，b和a完全相同，包括内存对齐填充的padding部分。
            let c = A(a.0, a.1); // 逐成员复制，非按位复制，c和a的padding部分不一定相同。
        }
    }

    mod _3 {
        #[derive(Debug, Copy, Clone)]
        struct A {
            a: u16,
            b: u8,
            c: bool,
        }

        fn main() {
            let a = unsound_a();
            // 尝试将 Some(a) 改为 a
            let some_a = Some(a);

            println!("a: {:#?}", a);
            println!("some_a: {:#?}", some_a);
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
            unsafe { *(&b as *const B as *const A) }
        }
    }

    // 示例三:
    mod _4 {
        use std::{ptr, mem};

        fn main() {
            let mut d = String::from("cccc");
            let d_len = d.len();
            // {
                let mut c = String::with_capacity(d_len);

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
            // }

            println!("{:?}", d.as_ptr());
            d.push_str("c");
            println!("{}", d);
        }
    }

    // 示例四: Copy 不一定只在栈上进行
    mod _5 {
        use std::cell::RefCell;

        fn main() {
            let a = Box::new(RefCell::new(1));
            let b = Box::new(RefCell::new(2));
            *b.borrow_mut() = *a.borrow();
            println!("b = {}", b.borrow());
        }
    }
}
