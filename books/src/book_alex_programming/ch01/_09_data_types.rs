#![allow(unused_variables)]

//! 09 | 语法面面观: 数据类型(一)
//! https://time.geekbang.org/course/detail/100060601-292838
//! 表达式的集合 -> 规则 / 类型系统 / 内存布局 / 内存安全 / 表达一致性 / 明确的语义 / 零成本抽象
//! 类型 & 行为
//!
//! 基本数据类型:  / isize, usize 指针大小 / 数组是二等公民 /
//! 字符串类型 ***
//! 胖指针
//! 切片类型
//!
//! 三种指针:
//!
//! 两种引用类型:
//!
//! 元组: 异构 / 单元元组
//!
//! Never类型: 底类型, 纳入集合 感叹号!
//!
//!
//! 结构体内存布局与内存对齐

pub fn data_type_01() {

}

/// 1.4: Expression-Oriented programming: const fib
mod const_fn_fib {
    // 用const声明的函数
    const fn gcd(a: u32, b: u32) -> u32 {
        match (a, b) {
            (x, 0) | (0, x) => x,

            (x, y) if x % 2 == 0 && y % 2 == 0 => 2*gcd(x/2, y/2),
            (x, y) | (y, x) if x % 2 == 0 => gcd(x/2, y),

            (x, y) if x < y => gcd((y-x)/2, x),
            (x, y) => gcd((x-y)/2, y),
        }
    }

    const fn fib(n: u128) -> u128 {
        const fn helper(n: u128, a: u128, b: u128, i: u128) -> u128 {
            if i <= n {
                helper(n, b, a + b, i + 1)
            } else {
                b
            }
        }
        helper(n, 1, 1, 2)
    }

    // cargo test book_alex_programming::ch01::_09_data_types::const_fn_fib::fib_test -- --nocapture
    // cargo-watch -x 'test book_alex_programming::ch01::_09_data_types::const_fn_fib::fib_test -- --nocapture'
    #[test]
    fn fib_test() {
        const X: u128 = fib(10);
        const GCD: u32 = gcd(21, 7);

        println!("{}", X);
        println!("{}", GCD);
    }

    const UNIT_TUPLE: [(u64, &str); 6] = {
        let mut i = 0;
        [
            (1 << (10 * { i += 1; i }), "KiB"),
            (1 << (10 * { i += 1; i }), "MiB"),
            (1 << (10 * { i += 1; i }), "GiB"),
            (1 << (10 * { i += 1; i }), "TiB"),
            (1 << (10 * { i += 1; i }), "PiB"),
            (1 << (10 * { i += 1; i }), "EiB")
        ]
    };

    const fn square_area(a: i32) -> i32 {
        let area = a * a;
        area
    }

    const AREA: i32 = square_area(5);

    #[test]
    fn dbg_test() {
        dbg!(UNIT_TUPLE);
        dbg!(AREA);
    }
}
