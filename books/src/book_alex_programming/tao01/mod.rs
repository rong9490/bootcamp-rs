#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! 第一章 | 新时代的语言
//! https://weread.qq.com/web/reader/0303203071848774030b9d6kc9f326d018c9f0f895fb5e4
//!
//!

mod duck_type {
	// 两个结构体实体
	pub struct Duck;
	pub struct Pig;

	// 行为Trait, 无默认实现
	pub trait Fly {
		fn fly(&self) -> bool;
	}

	impl Fly for Duck {
		fn fly(&self) -> bool {
			return true;
		}
	}

	impl Fly for Pig {
		fn fly(&self) -> bool {
			return false;
		}
	}
	// 静态分发: 泛型约束; 零成本抽象 fly_static
	fn fly_static<T: Fly>(s: T) -> bool {
	   s.fly()
	}
	// TODO ? 动态分发 (trait object)
	// fn fly_dyn(s: &Fly) -> bool {
	//     s.fly()
	// }

	#[test]
	fn main() {
		// turbofish绑定具体类型
		let pig = Pig;
		assert_eq!(fly_static::<Pig>(pig), false);
		let duck = Duck;
		assert_eq!(fly_static::<Duck>(duck), true);
	}
}
