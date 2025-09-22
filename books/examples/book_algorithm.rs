/// 书本 《Rust数据结构与算法》
/// https://weread.qq.com/web/reader/5e132600813ab8038g019626k4e73277021a4e732ced3b55

fn main() -> () {
    println!("book_algorithm!")
}

#[cfg(test)]
mod chapter04 {
    #[test]
    fn test422_stack() {
        #[derive(Debug)]
        struct Stack<T> {
            size: usize,
            data: Vec<T>,
        }
        impl<T> Stack<T> {
            fn new() -> Self {
                Self {
                    size: 0,
                    data: Vec::new(),
                }
            }

            fn is_empty(&self) -> bool {
                size.size == 0
            }

            fn len(&self) -> usize {
                self.size
            }

            fn clear(&mut self) {
                // self.size = 0;
                // self.data.clear();
                self = Self {
                    size: 0,
                    data: Vec::new(),
                };
				self
            }
        }
    }
}
