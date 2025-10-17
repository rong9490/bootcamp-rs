#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! 08 | 语法面面观: 面向表达式(三)
//! https://time.geekbang.org/course/detail/100060601-290168
//!
//! Rust中表达式的分类
//! 面向表达式 (Expression-Oriented programming)
//!
//! 位置表达式 与 值表达式
//! 位置上下文 与 值上下文
//! 绑定 binding
//! 静态变量初始化 / 解引用表达式 / 数组 / 字段 / 括号 / ...
//! 只有8个位置上下文
//!
//! 继承式可变 -> Shadowing
//! 内部可变性
//! 最佳实践: 用引用而非指针

/// 1.4: Expression_Oriented_programming
mod expression_oriented_programming {
    use std::collections::HashMap;

    // 可变引用, 解引用后直接在原地址上修改
    fn add_one(i: &mut u32) -> () {
        *i += 1;
    }

    // 不可变引用, 返回新变量
    fn plus_one(i: &u32) -> u32 {
        let i = i + 1;
        i
    }

    // cargo test book_alex_programming::ch01::_08_ctfe_expr::expression_oriented_programming::add_one_test -- --nocapture
    // cargo watch -x 'test book_alex_programming::ch01::_08_ctfe_expr::expression_oriented_programming::add_one_test -- --nocapture'
    #[test]
    fn add_one_test() {
        {
            let mut a = 41u32;
            add_one(&mut a); // 开始可变借用, 结束可变借用
            println!("{:?}", a); // 开始不可变借用, 结束不可变借用 (没有交叉)
            assert_eq!(a, 42)
        }

        {
            let a: u32 = 41u32;
            let b: u32 = plus_one(&a); // 各自都有自己的所有权, 没有发生转移
            println!("{:?}", b) ;
            assert_eq!(a, 41);
            assert_eq!(b, 42);
        }

        {
            let mut h: HashMap<&str, i32> = HashMap::new();
            h.insert("anwser", 42i32);
            println!("anwser is {:?}", h["anwser"]);
        }
    }

    fn position_move_and_copy() {
        let stack_a = 42; // a是位置表达式, 42实现了Copy, 所以是栈内存
        let stack_b = stack_a; // a出现在位置上下文, 发生了Copy行为
        // stack_a; // Okk

        let heap_a = "hello".to_string(); // a是未知表达式, 未实现Copy, 所以是堆内存
        let heap_b = heap_a; // a出现在值上下文, 发生了Move
        // heap_a; // Error!
    }
}

/**

    ### 笔记: 分号表达式 vs 块表达式

    1. 分号表达式返回值永远为自身的单元(Unit)类型：`()`
    2. 分号表达式只有在块表达式最后一行才会进行求值，其他时候只作为「连接符」存在
    3. 块表达式只对其最后一行表达式进行求值。

    ```
    fn main(){
        ;
        ;
        {
            ()
        }
        {
            ();
            use std::vec::Vec;
        }
        ();
        &();
        &{;}; // -> &()
        ; // ->  ()
    }
    ```
*/
pub fn semi_and_block_expr() {
    println!("1.4 : Semi vs Block ");
}
