#![allow(unused_variables)]
//! 第一章: Rust语言基础
//! CTFE 编译期函数求值 （compile time function evaluation）
//! 1.4 语法面面观 (二): 面向表达式 (中)

/**

    ### 必须是常量表达式才能在常量上下文使用
    # 声明宏示例

```rust
// (eval 1 + 2) 类型匹配
macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
```
*/

fn while_true_vs_loop() -> () {
    {
        let mut a;
        // 理解条件表达式: while (constexpr == true) {}
        // 保持一致性, 不能给while true开小灶, 统一不分析
        // 不能判断是否初始化了这个变量 --> 不安全, 统统不允许
        //
        #[allow(while_true)]
        while true {
            a = 1;
            break;
        }
        // used binding a is possibly-uninitialized
        // 无限循环, 理解编译器为什么这里会报错? 推荐使用 loop{} 循环
        //
        println!("{}", a);
    }
    {
        loop {
            // 无限循环..
        }
    }
}

/**
  const generic 常量泛型!
  #![feature(min_const_generics)]
  #![feature(const_in_array_repeat_expressions)]
  use core::mem::MayBeUninit;

  MayBeUninit 用于给泛型T生成一个未初始化的实例! 用于占位
  定义一个泛型结构体 ArrayVec, 泛型T 和 常量泛型 const N: usize

  #[derive(Debug)]
  pub struct ArrayVec<T, const N: usize> {
     items: [MaybeUninit<T>; N],
     length: usize,
  }

 */

fn array_const_generic() {
    // 常量泛型: const generic / 为什么需要常量泛型 ?
    // 数组是二等公民 -> 无法使用泛型来统一不同长度的数组
    let arr: [3; i32] = [1, 2, 3];
    let arr: [5; i32] = [1, 2, 3, 4, 5];
}

/**

# Path 展示

```rust
// 路径第一种用法: 模块路径: 嵌套关系, 父子关系
mod a {
    fn foo() {}
    mod b {
        mod c {
            fn foo() {
                // a > b > c 路径抽象与物理路径解耦
                super::super::foo(); // call a's foo function
                self::super::super::foo(); // call a's foo function
                // 关键字: super / self
            }
        }
    }
}

// 路径第二种用法: 方法调用
struct S;
impl S {
    fn f() { println!("S"); }
}
trait T1 {
    fn f() { println!("T1 f"); }
}
impl T1 for S {}
trait T2 {
    fn f() { println!("T2 f"); }
}
impl T2 for S {}
S::f();  // Calls the inherent impl.
// 完全限定无歧义调用
<S as T1>::f();  // Calls the T1 trait function.
<S as T2>::f();  // Calls the T2 trait function.


// 路径第三种用法: 泛型函数-turbofish操作符
// TurboFish 写法, 可以写通配符 ::<_> 自动推导
(0..10).collect::<Vec<_>>();
Vec::<u8>::with_capacity(1024);
```

*/

#[cfg(test)]
mod test {
    #[test]
    fn 标识符() {
        let thinking = "thinking";
        let thinking_ = "thinking 123";

        let _321_thinking = "thinking";

        // 变量名 utf-8, 后续逐步支持非utf-8, 便于专业领域的语义识别
        // non-ascii ident
        // let 🤣 = "Hello";

        // //! 模块级文档注释
        // //!! 同一行显式

        /* 跨行注释 */

        // /*! 同一行显式 */

        // 行级模块注释
    }

    fn 空白() {
        // \n \t tab
    }

    fn 词条() {
        // block / Expr / Stmt / Ident / Vis / ...写宏时很有用
    }

    fn 路径() {
        // 空间限定符::划分层级
    }

    // 面向表达式(一)
    // 表达式 -> 编译器计算的支持程度很高!
    // 从表达式的角度深入理解 变量和引用, 知其所以然
    // 声明语句 / 流程控制 / 表达式语句 / 宏语句(文本替换)
    #[test]
    fn 语法层面一致性() {

    }
}
