use anyhow::{Result, anyhow};
use std::ops::{Add, AddAssign, Deref, Mul};

// 自定义Vec数据结构
pub struct MyVector<T> {
    data: Vec<T>,
}

// 假设这是个耗时操作
// pretend this is a heavy operation, CPU intensive

// Copy + Default 拷贝与默认值
// Add<Output = T> 加号操作符
// AddAssign 加等号操作符
// Mul<Output = T> 乘号操作符

pub fn dot_product<T>(a: MyVector<T>, b: MyVector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    // a.len() 实际上是结构体解引用后 a.data.len(), 具体查看Deref Trait的实现
    if a.len() != b.len() || a.data.len() != b.data.len() {
        // a.len => a.data.len() (Deref trait)
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default(); // 获取该类型实现的默认值(Default Trait)
    let size: usize = a.len();
    for i in 0..size {
        sum += a[i] * b[i];
    }

    // 暂无法使用迭代器sum, 需要添加对应trait zip + map + sum
    // let sum = a.iter().zip(b.iter()).map(|(&a_i, &b_i)| a_i * b_i).sum();

    Ok(sum)
}

impl<T> Deref for MyVector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data // 自定义返回的内部数据
    }
}

impl<T> MyVector<T> {
    // 实现了 Into<Vec<T>> Trait, 就可以into转为 Vec<T>
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

// TODO
// Function std::thread::spawn
// F: FnOnce() -> T + Send + 'static

// pub fn spawn_major() {
//     let handle = std::thread::spawn(|| {
//         println!("Hello from the thread!");
//     });
// }
