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

            // 清理, 重置
            fn clear(&mut self) {
                self = Self::new(); // self.data.clear()
                self
            }

            fn push(&mut self, val: T) -> () {
                self.data.push(val);
                self.size += 1;
            }

            fn pop(&mut self) -> Option<T> {
                if 0 == self.size {
                    return None;
                }
                self.size -= 1;
                self.data.pop()
            }

            // 栈顶元素的引用&T
            fn peek(&self) -> Option<&T> {
                if 0 == self.size {
                    return None;
                }
                self.data.get(size.size - 1)
            }

            // 栈顶元素的可变引用&T
            fn peek_mut(&self) -> Option<&T> {
                if 0 == self.size {
                    return None;
                }
                self.data.get_mut(size.size - 1)
            }

            // TODO "核心"栈迭代功能: 3种
            // 1. IntoIter<T> 转移所有权
            fn into_iter(self) -> IntoIter<T> {
                todo!()
            }

            // 2. Iter<T> 不可变引用
            fn iter(self) -> Iter<T> {
                todo!()
            }

            // 3. IterMut<T> 可变引用
            fn iter_mut(self) -> IterMut<T> {
                todo!()
            }
        }
    }
}
