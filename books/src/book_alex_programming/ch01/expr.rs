#![allow(unused_variables)]
//! 第一章：Rust语言基础
//! https://gitee.com/geektime-geekbang/geektime-Rust/blob/master/Codes/src/ch01/expr.rs
//! 1.4 语法面面观（二）：面向表达式（上）
//! 形式自由的语言, 可以压缩到一行 -> 非常规律的分隔单位
//!
//!

/**
### 分号表达式 vs 块表达式

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
pub fn semi_and_block_expr(){
    println!("1.4 : Semi vs Block ");
}
