fn main() {
    println!("hello world");
}

#[derive(Debug)]
struct Stack<T> {
    size: usize,  // 栈大小
    data: Vec<T>, // 栈数据
}

// 为栈添加方法
impl<T> Stack<T> {
    // 实例化空栈
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }
}

// cargo test stack -- --nocapture
// cargo watch -x 'test --example stack -- --nocapture'

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let struct_type_size: usize = std::mem::size_of::<Stack<i32>>();
        println!("{}", struct_type_size);
        assert_eq!(struct_type_size, 32);
    }
}
