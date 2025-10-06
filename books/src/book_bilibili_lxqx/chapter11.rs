/*
    https://www.bilibili.com/video/BV1z8XpYKEG5
    pub trait Copy: Clone;
    要实现Copy必须同时实现Clone(Super Trait)
*/
use std::ops::Add;

// 自定义实现加号操作
fn myapp<T: Add<Output = T>>(a: T, b: T) -> T {
    todo!()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T = i32> {
    x: T,
    y: T,
}

// 为结构体实现impl
// 为结构体的引用实现impl

impl Add for Point {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// 对其不可变引用实现Clone, 而不是本身!
// 引出 earlyBound -> lateBound 类型单态化
// for<'a> &'a T: Add<Output = T>
#[derive(Debug)]
struct PointHeavy<T = i32> {
    x: T,
    y: T,
}

// TODO 重点理解!
impl Add for &PointHeavy {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}