#[macro_export]
macro_rules! my_vec {
    // 单个分支: $x 表达式次数不限 *
    ( $( $x:expr ),* ) => {
        {
            // type annotations needed cannot infer type
            // let mut temp_vec = Vec::<i32>::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec

            // 先分配一块内存, 不断push追加性能不是最好
            // Using the intrinsic produces a dramatic improvement in stack usage for
            // unoptimized programs using this code path to construct large Vecs.

            // 抄源码的写法
            <[_]>::into_vec(Box::new([$($x),+]))
        }
    };
}

#[macro_export]
macro_rules! my_vec2 {
    () => { Vec::<i32>::new() };
    // 额外匹配尾逗号, 不影响实际数据
    ( $( $x:expr ),+ $(,)? ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    ($elem: expr; $n: expr) => { std::vec::from_elem($elem, $n) };
}

// ? operator 问号操作符 -> 简化match匹配
#[macro_export]
macro_rules! my_try {
    ($x: expr) => {
        match $x {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    }
}