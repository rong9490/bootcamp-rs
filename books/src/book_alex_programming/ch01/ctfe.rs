// CTFE 编译期函数求值 （compile time function evaluation）

/**

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
}