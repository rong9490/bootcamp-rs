// 实现栈结构

// 如果是固定大小型
struct StackFixed<T, const N: usize> {
    size: usize,
    data: [T; N],
}

#[derive(Debug)]
pub struct Stack<T> {
    size: usize,  // 8字节(usize在win64上)
    data: Vec<T>, // 24字节(胖指针 = usize * 3)
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // 清理, 重置
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    pub fn push(&mut self, val: T) -> () {
        self.data.push(val);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if 0 == self.size {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    // 栈顶元素的引用&T
    pub fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }

    // 栈顶元素的可变引用&T
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    // TODO "核心"栈迭代功能: 3种
    // 1. IntoIter<T> 转移所有权
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // 2. Iter<T> 不可变引用
    pub fn iter(&self) -> Iter<T> {
        let mut iterator: Iter<'_, T> = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    // 3. IterMut<T> 可变引用
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator: IterMut<'_, T> = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

// 实现三种迭代功能
struct IntoIter<T>(Stack<T>); // Tuple型结构体
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T; // 关联类型
    // 迭代行为
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn mem() {
        // Struct类型大小与T无关, 固定的
        let mem_usize: usize = size_of::<usize>();
        let mem_stack: usize = mem_usize + mem_usize * 3;
        assert_eq!(mem_usize, 8);
        assert_eq!(mem_stack, 32);
        assert_eq!(size_of::<usize>(), mem_usize);
        assert_eq!(size_of::<Stack<bool>>(), mem_stack);
        assert_eq!(size_of::<Stack<char>>(), mem_stack);
        assert_eq!(size_of::<Stack<u8>>(), mem_stack);
        assert_eq!(size_of::<Stack<u16>>(), mem_stack);
        assert_eq!(size_of::<Stack<i32>>(), mem_stack);
        assert_eq!(size_of::<Stack<i64>>(), mem_stack);
    }

    #[test]
    fn basic() {
        let mut s: Stack<i32> = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        println!("size: {} {:?}", s.len(), s);
        println!("pop {:?}, size {}", s.pop().unwrap(), s.len());
        println!("empty: {}, {:?}", s.is_empty(), s);
    }
}
