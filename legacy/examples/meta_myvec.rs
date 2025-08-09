// my_vec! = myvec! { 1, 2, 3, 4, 5 } // Vec<i32>

#[macro_export]
macro_rules! my_vec {
    // 匹配 () 空参数
    () => {
        Vec::new()
    };
    // 支持 [1; 5] 这种语法
    // 由冒号分割的前后两个都是表达式, 第一个参数作为值, 第二个参数repeat多少次
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    // 匹配一个表达式
    // 匹配括号与不固定的参数个数 [] {} 都可以
    // * 表示重复任意次数 + 至少一次
    // 最后处理尾逗号
    ($($x:expr),+ $(,)?) => {
        {
            // 方案1: 运行时开辟内存
            // let mut temp_vec = Vec::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec

            // 方案2: 编译期处理
            <[_]>::into_vec(Box::new([$($x),+]))
        }
    };
}

fn main() {
    let v0 = vec![1, 2, 3, 4, 5]; // 阅读其源码, 查看区别
    println!("{:?}", v0);
    let v1: Vec<i32> = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v1);
    let v2: Vec<i32> = my_vec!{1, 2, 3, 4, 5};
    println!("{:?}", v2);
    let v3: Vec<i32> = my_vec![1; 5];
    println!("{:?}", v3);
    let v4: Vec<i32> = my_vec![1, 2, 3, 4, 5,]; // 处理末尾的逗号
    println!("{:?}", v4);

    // 这种写法是借用了 std::vec::from_elem 的实现, 但是性能更高
    // Box::new 是分配堆内存, 然后转成一个切片, 然后转成 Vec
    let v5 = <[_]>::into_vec(Box::new([1, 2, 3, 4, 5]));
    println!("{:?}", v5);
}

/* 

#[cfg(all(not(no_global_oom_handling), not(test)))]
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "vec_macro"]
#[allow_internal_unstable(rustc_attrs, liballoc_internals)]
macro_rules! vec {
    # 从上到下有多个匹配规则: 相当于重载, 匹配到第一个签名即可!

    # 1. 匹配 () 空参数, 返回一个空的 Vec
    () => (
        $crate::__rust_force_expr!($crate::vec::Vec::new())
    );

    # 2. 匹配 ($elem:expr; $n:expr) 一个参数
    ($elem:expr; $n:expr) => (
        $crate::__rust_force_expr!($crate::vec::from_elem($elem, $n))
    );

    # 3. 匹配 ($($x:expr),+ $(,)?) 多个参数
    ($($x:expr),+ $(,)?) => (
        $crate::__rust_force_expr!(<[_]>::into_vec(
            // This rustc_box is not required, but it produces a dramatic improvement in compile
            // time when constructing arrays with many elements.
            #[rustc_box]
            $crate::boxed::Box::new([$($x),+])
        ))
    );
}

*/